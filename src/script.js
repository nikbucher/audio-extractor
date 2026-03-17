// AudioSnip — Tauri frontend
// Implements UC-001 through UC-004

const { open } = window.__TAURI__.dialog;
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// ── State ──────────────────────────────────────────────────────────────────────

let videoPath = "";
let duration = 0; // seconds
let sp = 0; // start position 0–1
let ep = 1; // end position 0–1
let whole = false;
let fmt = "mp3";
let extracting = false;
let waveformData = [];
let dragInitDone = false;
let mediaServerPort = null;

// ── DOM refs ───────────────────────────────────────────────────────────────────

const dropZone = document.getElementById("drop-zone");
const browseBtn = document.getElementById("browse-btn");
const editor = document.getElementById("editor");
const fileNameEl = document.getElementById("file-name");
const fileMetaEl = document.getElementById("file-meta");
const loadNewBtn = document.getElementById("load-new-btn");
const video = document.getElementById("preview");
const videoScreen = document.getElementById("video-screen");
const vidOverlay = document.getElementById("vid-overlay");
const playRing = document.getElementById("play-ring");
const icoPlay = document.getElementById("ico-play");
const icoPause = document.getElementById("ico-pause");
const vidTs = document.getElementById("vid-ts");
const vidDur = document.getElementById("vid-dur");
const vidScrub = document.getElementById("vid-scrub");
const tStart = document.getElementById("t-start");
const tEnd = document.getElementById("t-end");
const timeRow = document.getElementById("time-row");
const wholeRow = document.getElementById("whole-row");
const chkbox = document.getElementById("chkbox");
const fmtChips = document.getElementById("fmt-chips");
const appendText = document.getElementById("append-text");
const extractBtn = document.getElementById("extract-btn");
const extractLabel = document.getElementById("extract-label");
const wfInfo = document.getElementById("wf-info");
const wfWrap = document.getElementById("wf-wrap");
const wfCanvas = document.getElementById("wf-canvas");
const hStart = document.getElementById("h-start");
const hEnd = document.getElementById("h-end");
const playhead = document.getElementById("playhead");
const wfTimes = document.getElementById("wf-times");
const wfLoading = document.getElementById("wf-loading");

// ── Helpers ────────────────────────────────────────────────────────────────────

function tf(s) {
	s = Math.round(s);
	const h = Math.floor(s / 3600);
	const m = Math.floor((s % 3600) / 60);
	const sc = s % 60;
	return h > 0
		? `${h}:${String(m).padStart(2, "0")}:${String(sc).padStart(2, "0")}`
		: `${m}:${String(sc).padStart(2, "0")}`;
}

function toHMS(s) {
	s = Math.round(s);
	const h = Math.floor(s / 3600);
	const m = Math.floor((s % 3600) / 60);
	const sc = s % 60;
	return [h, m, sc].map((v) => String(v).padStart(2, "0")).join(":");
}

function parseTime(str) {
	const p = str.trim().split(":").map(Number);
	if (p.some(isNaN)) return 0;
	if (p.length === 3) return p[0] * 3600 + p[1] * 60 + p[2];
	if (p.length === 2) return p[0] * 60 + p[1];
	return 0;
}

function formatBytes(bytes) {
	if (bytes >= 1073741824) return (bytes / 1073741824).toFixed(1) + " GB";
	if (bytes >= 1048576) return (bytes / 1048576).toFixed(0) + " MB";
	return (bytes / 1024).toFixed(0) + " KB";
}

// ── UC-001: File Selection ─────────────────────────────────────────────────────

async function openFileDialog() {
	try {
		const selected = await open({
			multiple: false,
			filters: [{ name: "Video", extensions: ["mp4", "mov", "avi", "mkv", "webm"] }],
		});
		if (selected) {
			await loadVideo(selected);
		}
	} catch (e) {
		console.error("Failed to open file:", e);
	}
}

async function loadVideo(path) {
	videoPath = path;
	const fileName = path.split(/[/\\]/).pop();

	// Get media server port
	if (!mediaServerPort) {
		mediaServerPort = await invoke("get_media_server_port");
	}

	// Get metadata from backend
	let meta;
	try {
		meta = await invoke("get_video_metadata", { path });
		duration = meta.duration_secs;
		fileNameEl.textContent = fileName;
		fileMetaEl.textContent = `${formatBytes(meta.file_size_bytes)} · ${tf(duration)}`;
	} catch (e) {
		alert("Failed to load video: " + e);
		return;
	}

	// Register path and set video source.
	// For formats Chromium can't play (AVI/MKV), the media server transcodes
	// to fMP4 in-memory and we use hls.js for playback.
	await invoke("register_media_path", { path: videoPath });
	if (meta.needs_transcode) {
		vidOverlay.classList.remove("hidden");
		loadViaHLS(videoPath);
	} else {
		video.src = `http://127.0.0.1:${mediaServerPort}/video?path=${encodeURIComponent(videoPath)}`;
	}

	// Reset state
	sp = 0;
	ep = 1;
	whole = false;
	chkbox.classList.remove("on");

	// Update scrubber
	vidScrub.max = Math.round(duration);
	vidScrub.value = 0;
	vidDur.textContent = tf(duration);
	vidTs.textContent = tf(0);

	// Show editor
	dropZone.style.display = "none";
	editor.style.display = "flex";

	// Clear previous waveform and show loading indicator
	waveformData = [];
	drawWave();
	wfLoading.classList.remove("hidden");

	// Load waveform
	try {
		waveformData = await invoke("get_audio_waveform", { path });
	} catch (e) {
		console.warn("Waveform unavailable:", e);
		waveformData = new Array(240).fill(0.5);
	}

	wfLoading.classList.add("hidden");
	setTimeout(() => {
		initDrag();
		refresh();
		renderTimeMarkers();
	}, 30);
}

let activeHls = null;

// ── HLS: progressive fMP4 streaming for AVI/MKV via hls.js ─────────────────

function loadViaHLS(path) {
	cleanupHLS();

	if (typeof Hls === "undefined" || !Hls.isSupported()) {
		console.error("[hls] hls.js not supported");
		vidOverlay.classList.add("hidden");
		return;
	}

	const hlsUrl = `http://127.0.0.1:${mediaServerPort}/hls?path=${encodeURIComponent(path)}`;

	const hls = new Hls({
		startPosition: 0, // Start from beginning, not live edge
		maxBufferLength: 60,
		maxMaxBufferLength: 120,
		maxBufferSize: 200 * 1024 * 1024,
	});

	activeHls = hls;
	hls.loadSource(hlsUrl);
	hls.attachMedia(video);

	let hlsRetries = 0;
	const MAX_HLS_RETRIES = 10;

	hls.on(Hls.Events.ERROR, (_event, data) => {
		console.warn(`[hls] ${data.type}: ${data.details}${data.fatal ? " (fatal)" : ""}`);
		if (data.fatal) {
			if (data.type === Hls.ErrorTypes.NETWORK_ERROR && hlsRetries < MAX_HLS_RETRIES) {
				hlsRetries++;
				// Playlist may be empty during early transcode — retry with backoff
				setTimeout(() => hls.startLoad(), 500 * hlsRetries);
			} else {
				console.error("[hls] giving up after", hlsRetries, "retries");
				hls.destroy();
				vidOverlay.classList.add("hidden");
			}
		}
	});
}

function cleanupHLS() {
	if (activeHls) {
		activeHls.destroy();
		activeHls = null;
	}
}

function loadNew() {
	vidOverlay.classList.add("hidden");
	cleanupHLS();
	editor.style.display = "none";
	dropZone.style.display = "";
	video.pause();
	video.removeAttribute("src");
	videoPath = "";
	duration = 0;
	updatePlayIcons(false);
}

// Drop zone events
dropZone.addEventListener("click", openFileDialog);
browseBtn.addEventListener("click", (e) => {
	e.stopPropagation();
	openFileDialog();
});
loadNewBtn.addEventListener("click", loadNew);

// Drag-and-drop via native Tauri file-drop events
const VIDEO_EXTS = ["mp4", "mov", "avi", "mkv", "webm"];
listen("tauri://drag-over", () => dropZone.classList.add("drag-over"));
listen("tauri://drag-leave", () => dropZone.classList.remove("drag-over"));
listen("tauri://drag-drop", (event) => {
	dropZone.classList.remove("drag-over");
	const paths = event.payload.paths || [];
	const file = paths.find((p) => VIDEO_EXTS.includes(p.split(".").pop().toLowerCase()));
	if (file) loadVideo(file);
});
// Prevent browser default so it doesn't navigate on drop
document.addEventListener("dragover", (e) => e.preventDefault());
document.addEventListener("drop", (e) => e.preventDefault());

// ── UC-002: Preview Selected Range ─────────────────────────────────────────────

// Waveform rendering
function drawWave() {
	if (!wfCanvas || !wfWrap) return;
	const W = wfWrap.offsetWidth;
	const H = 72;
	const dpr = devicePixelRatio || 1;
	wfCanvas.width = W * dpr;
	wfCanvas.height = H * dpr;
	const ctx = wfCanvas.getContext("2d");
	ctx.scale(dpr, dpr);
	ctx.clearRect(0, 0, W, H);

	// If no data yet (loading), just leave canvas cleared
	if (!waveformData.length) return;

	const dark = window.matchMedia("(prefers-color-scheme: dark)").matches;
	const inactive = dark ? "rgba(255,255,255,.14)" : "rgba(0,0,0,.11)";
	const data = waveformData;
	const n = data.length;
	const bw = Math.max(1.5, (W / n) * 0.56);

	// Selection highlight
	const x1 = whole ? 0 : sp * W;
	const x2 = whole ? W : ep * W;
	ctx.fillStyle = "rgba(186,117,23,.1)";
	ctx.fillRect(x1, 0, x2 - x1, H);

	// Bars
	for (let i = 0; i < n; i++) {
		const x = (i / n) * W + (W / n) * 0.5;
		const p = i / n;
		const inSelection = whole || (p >= sp && p <= ep);
		const bh = Math.max(2, data[i] * H * 0.82);
		ctx.fillStyle = inSelection ? "#BA7517" : inactive;
		ctx.beginPath();
		if (ctx.roundRect) ctx.roundRect(x - bw / 2, (H - bh) / 2, bw, bh, 1);
		else ctx.rect(x - bw / 2, (H - bh) / 2, bw, bh);
		ctx.fill();
	}
}

function updHandles() {
	hStart.style.left = (sp * 100).toFixed(2) + "%";
	hEnd.style.left = (ep * 100).toFixed(2) + "%";
	hStart.style.display = whole ? "none" : "";
	hEnd.style.display = whole ? "none" : "";
}

function updInputs() {
	const s = whole ? 0 : Math.round(sp * duration);
	const e = whole ? Math.round(duration) : Math.round(ep * duration);
	tStart.value = tf(s);
	tEnd.value = tf(e);
	wfInfo.textContent = tf(s) + " → " + tf(e);
	timeRow.style.opacity = whole ? ".4" : "1";
	timeRow.style.pointerEvents = whole ? "none" : "";
}

function refresh() {
	drawWave();
	updHandles();
	updInputs();
}

function renderTimeMarkers() {
	const count = 5;
	wfTimes.textContent = "";
	for (let i = 0; i < count; i++) {
		const span = document.createElement("span");
		span.className = "wf-t";
		span.textContent = tf((duration / (count - 1)) * i);
		wfTimes.appendChild(span);
	}
}

// Drag handles
function initDrag() {
	if (dragInitDone) return;
	dragInitDone = true;

	let dg = null;
	function mv(cx) {
		if (!dg) return;
		const r = wfWrap.getBoundingClientRect();
		const p = Math.max(0, Math.min(1, (cx - r.left) / r.width));
		if (dg === "s") sp = Math.min(p, ep - 0.005);
		else ep = Math.max(p, sp + 0.005);
		refresh();
	}

	hStart.addEventListener("mousedown", (e) => { dg = "s"; e.preventDefault(); });
	hEnd.addEventListener("mousedown", (e) => { dg = "e"; e.preventDefault(); });
	document.addEventListener("mousemove", (e) => { if (dg) mv(e.clientX); });
	document.addEventListener("mouseup", () => { dg = null; });

	hStart.addEventListener("touchstart", (e) => { dg = "s"; e.preventDefault(); }, { passive: false });
	hEnd.addEventListener("touchstart", (e) => { dg = "e"; e.preventDefault(); }, { passive: false });
	document.addEventListener("touchmove", (e) => { if (dg) mv(e.touches[0].clientX); }, { passive: false });
	document.addEventListener("touchend", () => { dg = null; });

	// Click on canvas to set playhead
	wfCanvas.addEventListener("click", (e) => {
		const r = wfWrap.getBoundingClientRect();
		const pos = Math.max(0, Math.min(1, (e.clientX - r.left) / r.width));
		video.currentTime = pos * duration;
	});
}

// Time input sync
tStart.addEventListener("change", timeChanged);
tEnd.addEventListener("change", timeChanged);

function timeChanged() {
	const s = parseTime(tStart.value);
	const e = parseTime(tEnd.value);
	sp = Math.max(0, Math.min(1, s / duration));
	ep = Math.max(0, Math.min(1, e / duration));
	if (ep <= sp) ep = Math.min(1, sp + 0.005);
	refresh();
}

// Extract whole checkbox
wholeRow.addEventListener("click", () => {
	whole = !whole;
	if (whole) chkbox.classList.add("on");
	else chkbox.classList.remove("on");
	refresh();
});

// Video playback — constrained to selected range (UC-002 main scenario)
function updatePlayIcons(isPlaying) {
	icoPlay.style.display = isPlaying ? "none" : "";
	icoPause.style.display = isPlaying ? "" : "none";
	playRing.classList.toggle("hidden", isPlaying);
}

videoScreen.addEventListener("click", () => {
	if (video.paused) {
		// If outside selected range, jump to start
		const rangeStart = whole ? 0 : sp * duration;
		const rangeEnd = whole ? duration : ep * duration;
		if (video.currentTime < rangeStart || video.currentTime >= rangeEnd) {
			video.currentTime = rangeStart;
		}
		video.play().catch((e) => { if (e.name !== "AbortError") console.warn("[video] play failed:", e.message); });
	} else {
		video.pause();
	}
});

video.addEventListener("canplay", () => vidOverlay.classList.add("hidden"));
video.addEventListener("loadeddata", () => vidOverlay.classList.add("hidden"));
video.addEventListener("error", () => console.error(`[video] error: code=${video.error?.code} msg=${video.error?.message}`));
video.addEventListener("play", () => updatePlayIcons(true));
video.addEventListener("pause", () => updatePlayIcons(false));

video.addEventListener("timeupdate", () => {
	const t = video.currentTime;

	// Constrain playback to selected range
	if (!video.paused && !whole) {
		const rangeEnd = ep * duration;
		if (t >= rangeEnd) {
			video.pause();
			video.currentTime = sp * duration;
		}
	}

	// Update scrubber and time display
	vidTs.textContent = tf(t);
	vidScrub.value = Math.round(t);

	// Update waveform playhead
	if (duration > 0) {
		playhead.style.left = ((t / duration) * 100).toFixed(2) + "%";
	}
});

vidScrub.addEventListener("input", () => {
	video.currentTime = Number(vidScrub.value);
});

// Resize handler (debounced)
let resizeTimer;
window.addEventListener("resize", () => {
	clearTimeout(resizeTimer);
	resizeTimer = setTimeout(() => {
		if (editor.style.display !== "none") drawWave();
	}, 80);
});

// ── UC-003: Extract Audio ──────────────────────────────────────────────────────

// Format chips
fmtChips.addEventListener("click", (e) => {
	const chip = e.target.closest(".fchip");
	if (!chip) return;
	fmt = chip.dataset.f;
	fmtChips.querySelectorAll(".fchip").forEach((c) => c.classList.toggle("on", c.dataset.f === fmt));
});

// Progress listener
listen("extraction-progress", (event) => {
	if (!extracting) return;
	const p = Math.round(event.payload);
	extractLabel.textContent = `Extracting... ${p}%`;
	extractBtn.style.background = `linear-gradient(to right, var(--color-text-primary) ${p}%, #94a3b8 ${p}%)`;
});

function setExtracting(busy) {
	extracting = busy;
	if (busy) {
		extractLabel.textContent = "Extracting... 0%";
		extractBtn.style.background = "linear-gradient(to right, var(--color-text-primary) 0%, #94a3b8 0%)";
		extractBtn.classList.add("extracting");
	} else {
		extractLabel.textContent = "Extract audio";
		extractBtn.style.background = "";
		extractBtn.classList.remove("extracting");
	}
}

extractBtn.addEventListener("click", async () => {
	// UC-004: Cancel if extracting
	if (extracting) {
		invoke("cancel_extraction");
		return;
	}

	if (!videoPath) {
		alert("Please select a video first.");
		return;
	}

	const append = appendText.value || "";

	setExtracting(true);
	try {
		if (whole) {
			await invoke("extract_whole_audio", {
				path: videoPath,
				format: fmt,
				append,
			});
		} else {
			const start = toHMS(sp * duration);
			const end = toHMS(ep * duration);
			await invoke("extract_audio_range", {
				path: videoPath,
				start,
				end,
				format: fmt,
				append,
			});
		}
		alert("Audio extracted!");
	} catch (e) {
		if (e !== "Cancelled") {
			alert("Extraction failed: " + e);
			console.error("Extraction error:", e);
		}
	} finally {
		setExtracting(false);
	}
});
