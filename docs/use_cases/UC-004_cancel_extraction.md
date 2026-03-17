# Use Case: Cancel Extraction

## Overview

**Use Case ID:** UC-004
**Use Case Name:** Cancel Extraction
**Primary Actor:** User
**Goal:** Stop an ongoing audio extraction before it completes
**Status:** Implemented

## Preconditions

- An audio extraction is in progress (UC-003 step 4).

## Main Success Scenario

1. User clicks the extract button while extraction is in progress.
2. System stops the extraction process.
3. System removes the partial output file.
4. System resets the extract button to its idle state.

## Alternative Flows

### A1: Extraction Completes Before Cancel Takes Effect

**Trigger:** The extraction finishes between the user clicking cancel and the system processing it (step 2).
**Flow:**

1. System completes the extraction normally.
2. The audio file is saved successfully.
3. System notifies the user that extraction is complete.

## Postconditions

### Success Postconditions

- The extraction process is stopped.
- No partial output file remains on disk.
- The application is ready for a new extraction.

### Failure Postconditions

- The extraction completes normally (A1).

## Business Rules

### BR-007: Partial File Cleanup

When an extraction is cancelled, any partially written output file must be deleted.
