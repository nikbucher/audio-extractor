// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::process::Command;

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
fn extract_audio_range(path: String, start: String, end: String, output: String, format: String) -> Result<(), String> {
    println!("[extract_audio_range] Called with:");
    println!("  path:   {}", path);
    println!("  start:  {}", start);
    println!("  end:    {}", end);
    println!("  output: {}", output);
    println!("  format: {}", format);

    let acodec = match format.as_str() {
        "aac" => {
            match get_audio_codec(&path).as_deref() {
                Some("aac") => "copy",
                _ => "aac",
            }
        },
        "mp3" => "libmp3lame",
        "ogg" => "libvorbis",
        _ => "copy", // fallback
    };

    let args = vec![
        "-y",
        "-i", &path,
        "-ss", &start,
        "-to", &end,
        "-vn",
        "-c:a", acodec,
        &output,
    ];

    println!("[extract_audio_range] Running: ffmpeg {:?}", args);
    run_ffmpeg_command(&args)
}

#[tauri::command]
fn extract_whole_audio(path: String, output: String, format: String) -> Result<(), String> {
    println!("[extract_whole_audio] Called with:");
    println!("  path:   {}", path);
    println!("  output: {}", output);
    println!("  format: {}", format);

    let acodec = match format.as_str() {
        "aac" => {
            match get_audio_codec(&path).as_deref() {
                Some("aac") => "copy",
                _ => "aac",
            }
        },
        "mp3" => "libmp3lame",
        "ogg" => "libvorbis",
        _ => "copy", // fallback
    };

    let args = vec![
        "-y",
        "-i", &path,
        "-vn",
        "-c:a", acodec,
        &output,
    ];

    println!("[extract_whole_audio] Running: ffmpeg {:?}", args);
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
