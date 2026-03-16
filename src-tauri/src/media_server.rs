use axum::{
	Router,
	body::Body,
	extract::Query,
	http::{HeaderMap, StatusCode, header},
	response::Response,
	routing::get,
};
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

const MAX_CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4 MB
const ALLOWED_EXTENSIONS: &[&str] = &["mp4", "mov", "avi", "mkv", "webm"];

#[derive(serde::Deserialize)]
struct FileQuery {
	path: String,
}

fn content_type_for(ext: &str) -> &'static str {
	match ext {
		"mp4" => "video/mp4",
		"webm" => "video/webm",
		"mov" => "video/quicktime",
		"avi" => "video/x-msvideo",
		"mkv" => "video/x-matroska",
		_ => "application/octet-stream",
	}
}

async fn serve_video(Query(query): Query<FileQuery>, headers: HeaderMap) -> Result<Response, StatusCode> {
	let file_path = Path::new(&query.path);
	let ext = file_path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()).unwrap_or_default();
	if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
		return Err(StatusCode::FORBIDDEN);
	}

	let file_size = tokio::fs::metadata(&query.path).await.map_err(|_| StatusCode::NOT_FOUND)?.len();
	let content_type = content_type_for(&ext);

	if let Some(range_value) = headers.get(header::RANGE) {
		let range_str = range_value.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
		let (start, end) = parse_range(range_str, file_size).ok_or(StatusCode::RANGE_NOT_SATISFIABLE)?;

		let chunk_end = end.min(start + MAX_CHUNK_SIZE - 1);
		let to_read = (chunk_end - start + 1) as usize;

		let mut file = tokio::fs::File::open(&query.path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
		file.seek(std::io::SeekFrom::Start(start)).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

		let mut buf = vec![0u8; to_read];
		let mut read = 0;
		while read < to_read {
			match file.read(&mut buf[read..]).await {
				Ok(0) => break,
				Ok(n) => read += n,
				Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
			}
		}
		buf.truncate(read);
		let actual_end = start + read as u64 - 1;

		Response::builder()
			.status(StatusCode::PARTIAL_CONTENT)
			.header(header::CONTENT_TYPE, content_type)
			.header(header::CONTENT_LENGTH, read)
			.header(header::CONTENT_RANGE, format!("bytes {}-{}/{}", start, actual_end, file_size))
			.header(header::ACCEPT_RANGES, "bytes")
			.body(Body::from(buf))
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
	} else {
		let file = tokio::fs::File::open(&query.path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
		let stream = tokio_util::io::ReaderStream::new(file);

		Response::builder()
			.status(StatusCode::OK)
			.header(header::CONTENT_TYPE, content_type)
			.header(header::CONTENT_LENGTH, file_size)
			.header(header::ACCEPT_RANGES, "bytes")
			.body(Body::from_stream(stream))
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
	}
}

fn parse_range(range: &str, file_size: u64) -> Option<(u64, u64)> {
	let range = range.strip_prefix("bytes=")?;
	let (start_str, end_str) = range.split_once('-')?;

	if start_str.is_empty() {
		let suffix: u64 = end_str.parse().ok()?;
		Some((file_size.checked_sub(suffix)?, file_size - 1))
	} else if end_str.is_empty() {
		let start: u64 = start_str.parse().ok()?;
		(start < file_size).then_some((start, file_size - 1))
	} else {
		let start: u64 = start_str.parse().ok()?;
		let end: u64 = end_str.parse().ok()?;
		(start <= end && start < file_size).then_some((start, end.min(file_size - 1)))
	}
}

/// Binds synchronously, spawns async serving. Returns the port immediately.
pub fn start() -> u16 {
	let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind media server");
	let port = listener.local_addr().unwrap().port();
	listener.set_nonblocking(true).unwrap();

	tauri::async_runtime::spawn(async move {
		let listener = tokio::net::TcpListener::from_std(listener).expect("Failed to convert listener");
		let app = Router::new().route("/video", get(serve_video));
		println!("[media_server] Listening on 127.0.0.1:{}", port);
		axum::serve(listener, app).await.expect("Media server error");
	});

	port
}
