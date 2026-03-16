# Use Case: Select Video File

## Overview

**Use Case ID:** UC-001  
**Use Case Name:** Select Video File  
**Primary Actor:** User  
**Goal:** Load a video file into the application for audio extraction  
**Status:** Draft  

## Preconditions

- The application is open.

## Main Success Scenario

1. User clicks the file selection button.
2. System opens a file picker filtered to supported video formats (MP4, MOV, AVI, MKV, WebM).
3. User selects a video file.
4. System loads the video and displays it in the preview area.
5. System sets the from/to range handles to cover the full video duration.

## Alternative Flows

### A1: User Cancels File Picker

**Trigger:** User closes the file picker without selecting a file (step 3).
**Flow:**

1. System returns to its previous state with no changes.

### A2: Video Already Loaded

**Trigger:** A video is already loaded when the user selects a new file (step 1).
**Flow:**

1. System replaces the current video with the newly selected one.
2. System resets the from/to range handles to the new video's full duration.
3. Use case continues at step 4.

### A3: Unsupported File Format

**Trigger:** The selected file is not a supported video format (step 3).
**Flow:**

1. System displays an error message indicating the format is not supported.

### A4: File Cannot Be Loaded

**Trigger:** The file exists but cannot be read or played (step 4).
**Flow:**

1. System displays an error message.

## Postconditions

### Success Postconditions

- The video is displayed in the preview area.
- The from/to range covers the full video duration.
- The user can proceed to preview or extract audio.

### Failure Postconditions

- No video is loaded (or the previously loaded video remains).
- System displays an appropriate error message.

## Business Rules

### BR-001: Supported Video Formats

Only MP4, MOV, AVI, MKV, and WebM files are accepted.
