# Use Case: Serve Video Preview

## Overview

**Use Case ID:** UC-005
**Use Case Name:** Serve Video Preview
**Primary Actor:** Browser (Video Element)
**Goal:** Provide a playable video stream for in-app preview
**Status:** Draft

## Preconditions

- The user has selected a video file (UC-001 step 3).
- The file path is registered with the media server.
- The local media server is running.

## Main Success Scenario

1. Browser requests the video stream from the media server.
2. System verifies the file path is registered and the file extension is allowed.
3. System determines the video container format.
4. System serves the file directly with range-request support.
5. Browser receives the video data and begins playback.

## Alternative Flows

### A1: Container Requires Transcoding

**Trigger:** The video container is not natively playable by the browser (step 3).
**Flow:**

1. System displays a loading indicator over the preview area.
2. System begins transcoding the video to a streamable format.
3. System streams the transcoded data progressively to the browser as it becomes available.
4. Browser begins playback once enough data is received.
5. System hides the loading indicator.

### A2: Transcoding Already Complete

**Trigger:** The same video was previously requested and transcoding has finished (step 3).
**Flow:**

1. System serves the transcoded data from its in-memory buffer with range-request support.
2. Use case continues at step 5.

### A3: Different Video Replaces Active Transcode

**Trigger:** A new video is loaded while a transcode is still in progress (step 1).
**Flow:**

1. System aborts the active transcode.
2. System discards the previous transcode buffer.
3. Use case continues at step 2.

### A4: Path Not Registered

**Trigger:** The requested file path was not registered by the application (step 2).
**Flow:**

1. System rejects the request.

### A5: Transcoding Fails

**Trigger:** The transcode process exits with an error (A1 step 2).
**Flow:**

1. System marks the transcode as complete with no data.
2. Browser receives an empty response and displays an error.

### A6: Browser Requests Byte Range

**Trigger:** Browser sends a range request for seeking or buffering (step 4 or A2 step 1).
**Flow:**

1. System reads the requested byte range from the file or transcode buffer.
2. System responds with the partial content and range metadata.
3. Browser uses the data for seeking or continued playback.

## Postconditions

### Success Postconditions

- The video is playable in the browser's video element.
- The browser can seek within the video via range requests.

### Failure Postconditions

- No video data is delivered.
- The preview area shows an error or remains empty.

## Business Rules

### BR-008: Transcode-Required Containers

Container formats that the browser cannot play natively (AVI, MKV) are transcoded to fragmented MP4 for preview.

### BR-009: Codec-Aware Transcoding Strategy

When the source video codec is natively supported by the browser (H.264, VP8, VP9, AV1), the video stream is remuxed without re-encoding. Otherwise, the video is re-encoded.

### BR-010: Path Registration

The media server only serves files that the application has explicitly registered. Unregistered paths are rejected.
