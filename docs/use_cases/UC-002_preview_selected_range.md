# Use Case: Preview Selected Range

## Overview

**Use Case ID:** UC-002
**Use Case Name:** Preview Selected Range
**Primary Actor:** User
**Goal:** Define and preview a time range to verify the audio selection before extracting
**Status:** Draft

## Preconditions

- A video file is loaded (UC-001 completed successfully).
- The from/to range handles are visible and set to the full video duration.

## Main Success Scenario

1. User drags the "from" handle to set the start of the range.
2. User drags the "to" handle to set the end of the range.
3. User plays the video.
4. System plays the video constrained to the selected range.
5. User confirms the selection covers the desired audio segment.

## Alternative Flows

### A1: Adjust Range After Previewing

**Trigger:** The previewed range doesn't match what the user wants (step 5).
**Flow:**

1. User adjusts the from and/or to handles.
2. Use case continues at step 3.

### A2: Extract Whole Audio

**Trigger:** User wants the full audio track (step 1).
**Flow:**

1. User leaves the from/to handles at their default positions (full duration).
2. Use case ends — no preview adjustment needed.

### A3: Type Time Values Manually

**Trigger:** User knows exact timestamps (step 1).
**Flow:**

1. User enters start and end times directly in the time input fields.
2. System updates the handle positions to match.
3. Use case continues at step 3.

## Postconditions

### Success Postconditions

- The from/to range reflects the user's desired audio segment.
- The user has verified the selection through playback.

### Failure Postconditions

- The range remains at its previous position.

## Business Rules

### BR-002: Time Format

Time values are displayed and entered in HH:MM:SS format.

### BR-003: Range Constraints

- The "from" time must be before the "to" time.
- Both values must be within the video's duration.
