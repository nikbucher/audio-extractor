// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::Path;
use std::process::Command;

const ALLOWED_FORMATS: &[&str] = &["aac", "mp3", "ogg"];
const ALLOWED_VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "avi", "mkv", "webm"];

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

fn resolve_codec(format: &str, path: &str) -> &'static str {
    match format {
        "aac" => {
            match get_audio_codec(path).as_deref() {
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

// Helper to get audio codec using ffprobe
fn get_audio_codec(path: &str) -> Option<String> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "a:0",
            "-show_entries", "stream=codec_name",
            "-of", "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output();
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

#[tauri::command]
fn extract_audio_range(path: String, start: String, end: String, format: String, append: String) -> Result<(), String> {
    validate_path(&path)?;
    validate_format(&format)?;
    validate_time(&start)?;
    validate_time(&end)?;

    let output = build_output_path(&path, &format, &append)?;
    let acodec = resolve_codec(&format, &path);

    println!("[extract_audio_range] path={}, start={}, end={}, output={}", path, start, end, output);

    let args = vec![
        "-y",
        "-i", &path,
        "-ss", &start,
        "-to", &end,
        "-vn",
        "-c:a", acodec,
        &output,
    ];

    run_ffmpeg_command(&args)
}

#[tauri::command]
fn extract_whole_audio(path: String, format: String, append: String) -> Result<(), String> {
    validate_path(&path)?;
    validate_format(&format)?;

    let output = build_output_path(&path, &format, &append)?;
    let acodec = resolve_codec(&format, &path);

    println!("[extract_whole_audio] path={}, output={}", path, output);

    let args = vec![
        "-y",
        "-i", &path,
        "-vn",
        "-c:a", acodec,
        &output,
    ];

    run_ffmpeg_command(&args)
}

fn run_ffmpeg_command(args: &[&str]) -> Result<(), String> {
    let result = Command::new("ffmpeg")
        .args(args)
        .output();
    match result {
        Ok(output) => {
            println!("[ffmpeg] exited with status: {}", output.status);
            if !output.status.success() {
                println!("[ffmpeg] stderr: {}", String::from_utf8_lossy(&output.stderr));
                return Err(format!("FFmpeg failed: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
        Err(e) => {
            eprintln!("[ffmpeg] Failed to run ffmpeg: {}. Is FFmpeg installed and in your PATH?", e);
            return Err(format!("FFmpeg failed: {}. Is FFmpeg installed and in your PATH?", e));
        }
    }
    println!("[ffmpeg] Success!");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![extract_audio_range, extract_whole_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
