# Use Case: Extract Audio

## Overview

**Use Case ID:** UC-003
**Use Case Name:** Extract Audio
**Primary Actor:** User
**Goal:** Extract the audio from the selected video range and save it as an audio file
**Status:** Draft

## Preconditions

- A video file is loaded (UC-001 completed successfully).
- The from/to range is set (either full duration or a user-defined range via UC-002).

## Main Success Scenario

1. User selects an output format (AAC, MP3, or OGG).
2. User optionally enters a text suffix for the output filename.
3. User clicks the extract button.
4. System begins extracting audio and displays a progress indicator.
5. System saves the audio file to the same folder as the source video.
6. System notifies the user that extraction is complete.

## Alternative Flows

### A1: FFmpeg Not Installed

**Trigger:** System cannot find FFmpeg on the machine (step 4).
**Flow:**

1. System displays an error message indicating FFmpeg is required.

### A2: Extraction Fails

**Trigger:** FFmpeg exits with an error (step 4).
**Flow:**

1. System displays the error details to the user.
2. System resets the extract button to its idle state.

### A3: Choose Output Location

**Trigger:** User wants to save to a different folder (step 3).
**Flow:**

1. User changes the output location before extracting.
2. Use case continues at step 3.

### A4: Extract Another Clip

**Trigger:** User wants multiple clips from the same video (step 6).
**Flow:**

1. User adjusts the from/to range (UC-002).
2. Use case restarts at step 1.

## Postconditions

### Success Postconditions

- An audio file exists at the output location in the chosen format.
- The filename is derived from the source video name, optional suffix, and format extension.

### Failure Postconditions

- No audio file is created (or partial file is cleaned up).
- System displays an error message.

## Business Rules

### BR-004: Supported Output Formats

The user can choose from AAC, MP3, or OGG.

### BR-005: Output Filename Convention

The output file is named `{video-stem}{-suffix}-audio.{format}`, saved in the same folder as the source video by default.

### BR-006: Codec Optimization

When the source audio codec matches the output format (e.g., AAC to AAC), the audio stream is copied without re-encoding.
