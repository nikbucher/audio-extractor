const { open } = window.__TAURI__.dialog;
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const mediaServerPort = invoke("get_media_server_port");

const videoInput = document.getElementById("videoInput");
const video = document.getElementById("preview");
const startInput = document.getElementById("start");
const endInput = document.getElementById("end");
const extractWholeCheckbox = document.getElementById("extractWhole");
const formatSelect = document.getElementById("format");
const appendTextInput = document.getElementById("appendText");
const startGroup = document.getElementById("start-group");
const endGroup = document.getElementById("end-group");
const extractBtn = document.getElementById("extract");
let videoPath = "";
let extracting = false;

function secondsToHMS(seconds) {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  return [h, m, s].map(v => v.toString().padStart(2, '0')).join(":");
}

videoInput.addEventListener("click", async () => {
  console.log("Opening file dialog...");
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Video', extensions: ['mp4', 'mov', 'avi', 'mkv', 'webm'] }]
    });
    if (selected) {
      videoPath = selected;
      const port = await mediaServerPort;
      video.src = `http://127.0.0.1:${port}/video?path=${encodeURIComponent(videoPath)}`;
      console.log("Selected video:", videoPath);
      // Wait for metadata to load so we can get duration
      video.onloadedmetadata = () => {
        const duration = video.duration;
        console.log("Video duration:", duration);
        if (!isNaN(duration)) {
          const durationFormatted = secondsToHMS(duration);
          startInput.value = "00:00:00";
          endInput.value = durationFormatted;
          console.log(`Video duration: ${duration}s, ${durationFormatted}`);
        }
      };
    }
  } catch (e) {
    console.error("Failed to open file:", e);
  }
});

extractWholeCheckbox.addEventListener("change", () => {
  // Toggle visibility of start and end time controls based on checkbox state
  const show = !extractWholeCheckbox.checked;
  startGroup.style.display = show ? "flex" : "none";
  endGroup.style.display = show ? "flex" : "none";
});

function setExtracting(busy) {
  extracting = busy;
  if (busy) {
    extractBtn.textContent = "Extracting... 0%";
    extractBtn.style.background = `linear-gradient(to right, var(--primary-color) 0%, #94a3b8 0%)`;
    extractBtn.classList.add("extracting");
  } else {
    extractBtn.textContent = "\u{1F50A} Extract Audio";
    extractBtn.style.background = "";
    extractBtn.classList.remove("extracting");
  }
}

function updateProgress(percent) {
  const p = Math.round(percent);
  extractBtn.textContent = `Extracting... ${p}%`;
  extractBtn.style.background = `linear-gradient(to right, var(--primary-color) ${p}%, #94a3b8 ${p}%)`;
}

listen("extraction-progress", (event) => {
  updateProgress(event.payload);
});

extractBtn.addEventListener("click", async () => {
  if (extracting) {
    invoke("cancel_extraction");
    return;
  }

  if (!videoPath) {
    alert("Please select a video first.");
    return;
  }

  const extractWhole = extractWholeCheckbox.checked;
  const format = formatSelect.value;
  const append = appendTextInput.value || "";

  setExtracting(true);
  try {
    if (extractWhole) {
      await invoke("extract_whole_audio", {
        path: videoPath,
        format,
        append,
      });
    } else {
      const start = startInput.value;
      const end = endInput.value;
      if (!start || !end) {
        alert("Please specify start/end times or check 'Extract whole audio'.");
        return;
      }
      await invoke("extract_audio_range", {
        path: videoPath,
        start,
        end,
        format,
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
