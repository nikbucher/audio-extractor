// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod media_server;

use std::path::Path;
use std::process::Stdio;
use tauri::Emitter;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};

static MEDIA_SERVER_PORT: AtomicU16 = AtomicU16::new(0);
static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn get_media_server_port() -> u16 {
    MEDIA_SERVER_PORT.load(Ordering::Relaxed)
}

#[tauri::command]
fn cancel_extraction() {
    CANCEL_FLAG.store(true, Ordering::Relaxed);
    println!("[cancel] Extraction cancel requested");
}

const ALLOWED_FORMATS: &[&str] = &["aac", "mp3", "ogg"];
const ALLOWED_VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "avi", "mkv", "webm"];

/// Find a command binary by checking common installation paths.
/// On macOS, .app bundles launched from Finder have a restricted PATH that
/// doesn't include Homebrew paths, so we search known locations explicitly.
fn find_command(name: &str) -> String {
    let candidates: &[&str] = if cfg!(target_os = "macos") {
        &["/opt/homebrew/bin", "/usr/local/bin"]
    } else if cfg!(target_os = "linux") {
        &["/usr/local/bin", "/usr/bin"]
    } else {
        &[]
    };

    for dir in candidates {
        let full = format!("{}/{}", dir, name);
        if Path::new(&full).exists() {
            return full;
        }
    }

    name.to_string()
}

fn validate_path(path: &str) -> Result<(), String> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(format!("File not found: {}", path));
    }
    match p.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
        Some(ext) if ALLOWED_VIDEO_EXTENSIONS.contains(&ext.as_str()) => Ok(()),
        _ => Err(format!("Unsupported video format: {}", path)),
    }
}

fn validate_format(format: &str) -> Result<(), String> {
    if ALLOWED_FORMATS.contains(&format) {
        Ok(())
    } else {
        Err(format!("Unsupported output format: {}", format))
    }
}

fn validate_time(time: &str) -> Result<(), String> {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid time format (expected HH:MM:SS): {}", time));
    }
    for part in &parts {
        if part.parse::<u32>().is_err() {
            return Err(format!("Invalid time format (expected HH:MM:SS): {}", time));
        }
    }
    Ok(())
}

fn hms_to_seconds(hms: &str) -> Result<f64, String> {
    let parts: Vec<&str> = hms.split(':').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid time format: {}", hms));
    }
    let h: f64 = parts[0].parse().map_err(|_| format!("Invalid hours: {}", parts[0]))?;
    let m: f64 = parts[1].parse().map_err(|_| format!("Invalid minutes: {}", parts[1]))?;
    let s: f64 = parts[2].parse().map_err(|_| format!("Invalid seconds: {}", parts[2]))?;
    Ok(h * 3600.0 + m * 60.0 + s)
}

async fn resolve_codec(format: &str, path: &str) -> &'static str {
    match format {
        "aac" => {
            match get_audio_codec(path).await.as_deref() {
                Some("aac") => "copy",
                _ => "aac",
            }
        },
        "mp3" => "libmp3lame",
        "ogg" => "libvorbis",
        _ => "copy",
    }
}

fn build_output_path(path: &str, format: &str, append: &str) -> Result<String, String> {
    let p = Path::new(path);
    let dir = p.parent().ok_or("Cannot determine parent directory")?;
    let stem = p.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Cannot determine filename")?;
    let suffix = if append.is_empty() {
        String::new()
    } else {
        format!("-{}", append)
    };
    let output = dir.join(format!("{}{}-audio.{}", stem, suffix, format));
    output.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid output path".to_string())
}

async fn get_audio_codec(path: &str) -> Option<String> {
    let output = Command::new(find_command("ffprobe"))
        .args([
            "-v", "error",
            "-select_streams", "a:0",
            "-show_entries", "stream=codec_name",
            "-of", "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()
        .await;
    let output = match output {
        Ok(out) => out,
        Err(e) => {
            eprintln!("[ffprobe] Failed to run ffprobe: {}. Is FFmpeg/FFprobe installed and in your PATH?", e);
            return None;
        }
    };
    if !output.status.success() {
        return None;
    }
    let codec = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if codec.is_empty() {
        None
    } else {
        Some(codec)
    }
}

async fn get_duration(path: &str) -> Option<f64> {
    let output = Command::new(find_command("ffprobe"))
        .args([
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()
        .await;
    let output = match output {
        Ok(out) => out,
        Err(e) => {
            eprintln!("[ffprobe] Failed to get duration: {}", e);
            return None;
        }
    };
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f64>()
        .ok()
}

#[tauri::command]
async fn extract_audio_range(app: tauri::AppHandle, path: String, start: String, end: String, format: String, append: String) -> Result<(), String> {
    validate_path(&path)?;
    validate_format(&format)?;
    validate_time(&start)?;
    validate_time(&end)?;

    let output = build_output_path(&path, &format, &append)?;
    let acodec = resolve_codec(&format, &path).await;
    let duration_secs = hms_to_seconds(&end)? - hms_to_seconds(&start)?;

    CANCEL_FLAG.store(false, Ordering::Relaxed);
    println!("[extract_audio_range] path={}, start={}, end={}, output={}", path, start, end, output);

    let args = vec![
        "-y",
        "-i", &path,
        "-ss", &start,
        "-to", &end,
        "-vn",
        "-c:a", acodec,
        "-progress", "pipe:1",
        &output,
    ];

    run_ffmpeg_command(&args, duration_secs, &output, &app).await
}

#[tauri::command]
async fn extract_whole_audio(app: tauri::AppHandle, path: String, format: String, append: String) -> Result<(), String> {
    validate_path(&path)?;
    validate_format(&format)?;

    let output = build_output_path(&path, &format, &append)?;
    let acodec = resolve_codec(&format, &path).await;
    let duration_secs = get_duration(&path).await.unwrap_or(0.0);

    CANCEL_FLAG.store(false, Ordering::Relaxed);
    println!("[extract_whole_audio] path={}, output={}", path, output);

    let args = vec![
        "-y",
        "-i", &path,
        "-vn",
        "-c:a", acodec,
        "-progress", "pipe:1",
        &output,
    ];

    run_ffmpeg_command(&args, duration_secs, &output, &app).await
}

async fn run_ffmpeg_command(args: &[&str], duration_secs: f64, output_path: &str, app: &tauri::AppHandle) -> Result<(), String> {
    let mut child = Command::new(find_command("ffmpeg"))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("FFmpeg failed: {}. Is FFmpeg installed and in your PATH?", e))?;

    let mut cancelled = false;

    // Read progress from stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if CANCEL_FLAG.load(Ordering::Relaxed) {
                println!("[ffmpeg] Cancel requested, killing process");
                let _ = child.kill().await;
                cancelled = true;
                break;
            }
            if let Some(time_us_str) = line.strip_prefix("out_time_us=") {
                if duration_secs > 0.0 {
                    if let Ok(time_us) = time_us_str.parse::<f64>() {
                        let progress = ((time_us / 1_000_000.0) / duration_secs * 100.0).min(100.0);
                        let _ = app.emit("extraction-progress", progress);
                    }
                }
            }
        }
    }

    if cancelled {
        // Clean up partial output file
        let _ = tokio::fs::remove_file(output_path).await;
        return Err("Cancelled".to_string());
    }

    let output = child.wait_with_output().await
        .map_err(|e| format!("FFmpeg failed: {}", e))?;

    println!("[ffmpeg] exited with status: {}", output.status);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("[ffmpeg] stderr: {}", stderr);
        return Err(format!("FFmpeg failed: {}", stderr));
    }

    let _ = app.emit("extraction-progress", 100.0_f64);
    println!("[ffmpeg] Success!");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            let port = media_server::start();
            MEDIA_SERVER_PORT.store(port, Ordering::Relaxed);
            println!("[setup] Media server started on port {}", port);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            extract_audio_range,
            extract_whole_audio,
            cancel_extraction,
            get_media_server_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
