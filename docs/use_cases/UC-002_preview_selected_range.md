# Use Case: Preview Selected Range

## Overview

**Use Case ID:** UC-002
**Use Case Name:** Preview Selected Range
**Primary Actor:** User
**Goal:** Define and preview a time range to verify the audio selection before extracting
**Status:** Tested

## Preconditions

- A video file is loaded (UC-001 completed successfully).
- The audio waveform is displayed in the timeline.
- The from/to range handles are visible and set to the full video duration.

## Main Success Scenario

1. User drags the "from" handle on the waveform timeline to set the start of the range.
2. User drags the "to" handle on the waveform timeline to set the end of the range.
3. System highlights the selected region on the waveform and updates the time display.
4. User plays the video.
5. System plays the video constrained to the selected range.
6. System displays a playhead on the waveform that tracks the current playback position.
7. System pauses playback and resets the playhead when the "to" boundary is reached.
8. User confirms the selection covers the desired audio segment.

## Alternative Flows

### A1: Adjust Range After Previewing

**Trigger:** The previewed range doesn't match what the user wants (step 8).
**Flow:**

1. User adjusts the from and/or to handles.
2. Use case continues at step 3.

### A2: Extract Whole Audio

**Trigger:** User wants the full audio track (step 1).
**Flow:**

1. User checks the "Extract whole audio" checkbox.
2. System hides the range handles and highlights the entire waveform.
3. Use case ends — no preview adjustment needed.

### A3: Type Time Values Manually

**Trigger:** User knows exact timestamps (step 1).
**Flow:**

1. User enters start and end times directly in the time input fields.
2. System updates the handle positions and waveform highlight to match.
3. Use case continues at step 4.

### A4: Seek via Waveform

**Trigger:** User wants to jump to a specific position (step 4).
**Flow:**

1. User clicks a position on the waveform timeline.
2. System moves the video playback position to the clicked time.
3. Use case continues at step 5.

### A5: Seek via Scrubber

**Trigger:** User wants to jump to a specific position (step 4).
**Flow:**

1. User drags the video scrubber below the preview.
2. System moves the video playback position to the selected time.
3. Use case continues at step 5.

## Postconditions

### Success Postconditions

- The from/to range reflects the user's desired audio segment.
- The selected range is visually highlighted on the waveform.
- The user has verified the selection through playback.

### Failure Postconditions

- The range remains at its previous position.

## Business Rules

### BR-002: Time Format

Time values are transmitted in HH:MM:SS format. The display may use a shorter notation (e.g., M:SS) when hours are zero.

### BR-003: Range Constraints

- The "from" time must be before the "to" time.
- Both values must be within the video's duration.
