# Test–Use Case Traceability Matrix

Tracks whether every scenario and business rule from the use case specs has test coverage.
Run filtered tests with `cargo test uc001`, `cargo test uc002`, etc.

## Scenario Coverage

### UC-001: Select Video File

| Scenario                     | Tested | Tests                                                                                               |
|------------------------------|--------|-----------------------------------------------------------------------------------------------------|
| Main Success                 | Yes    | `uc001_validate_extension_accepts_supported`, `uc001_validate_path_accepts_valid_video`             |
| A1: User Cancels File Picker | No     | Frontend-only, no backend logic                                                                     |
| A2: Video Already Loaded     | No     | Frontend-only state management                                                                      |
| A3: Unsupported File Format  | Yes    | `uc001_validate_extension_rejects_unsupported`, `uc001_validate_path_rejects_unsupported_extension` |
| A4: File Cannot Be Loaded    | Yes    | `uc001_validate_path_rejects_missing_file`                                                          |

### UC-002: Preview Selected Range

| Scenario                          | Tested | Tests                                                                                                 |
|-----------------------------------|--------|-------------------------------------------------------------------------------------------------------|
| Main Success                      | Yes    | `uc002_validate_time_accepts_valid_hms`, `uc002_hms_to_seconds_*` (3), `uc002_range_start_before_end` |
| A1: Adjust Range After Previewing | No     | Frontend-only interaction                                                                             |
| A2: Extract Whole Audio           | No     | Covered implicitly by UC-003 `extract_whole_audio` command                                            |
| A3: Type Time Values Manually     | Yes    | `uc002_validate_time_rejects_*` (4)                                                                   |

### UC-003: Extract Audio

| Scenario                   | Tested  | Tests                                                                                                      |
|----------------------------|---------|------------------------------------------------------------------------------------------------------------|
| Main Success               | Yes     | `uc003_validate_format_accepts_supported`, `uc003_build_output_path_*` (3), `uc003_codec_for_format_*` (4) |
| A1: FFmpeg Not Installed   | No      | Requires integration test with missing binary                                                              |
| A2: Extraction Fails       | Partial | `uc003_validate_format_rejects_unsupported` — format rejection tested, FFmpeg error path not tested        |
| A3: Choose Output Location | No      | Not yet implemented                                                                                        |
| A4: Extract Another Clip   | No      | Frontend-only workflow                                                                                     |

### UC-004: Cancel Extraction

| Scenario                               | Tested | Tests                                  |
|----------------------------------------|--------|----------------------------------------|
| Main Success                           | Yes    | `uc004_cancel_flag_store_and_load`     |
| A1: Extraction Completes Before Cancel | No     | Race condition, difficult to unit test |

## Business Rule Coverage

| Rule   | Description                | UC     | Tested  | Tests                                                                                                                     |
|--------|----------------------------|--------|---------|---------------------------------------------------------------------------------------------------------------------------|
| BR-001 | Supported Video Formats    | UC-001 | Yes     | `uc001_validate_extension_accepts_supported`, `uc001_validate_extension_rejects_unsupported`, `uc001_validate_path_*` (3) |
| BR-002 | Time Format (HH:MM:SS)     | UC-002 | Yes     | `uc002_validate_time_*` (5), `uc002_hms_to_seconds_*` (3)                                                                 |
| BR-003 | Range Constraints          | UC-002 | Partial | `uc002_range_start_before_end` — "within duration" not tested                                                             |
| BR-004 | Supported Output Formats   | UC-003 | Yes     | `uc003_validate_format_accepts_supported`, `uc003_validate_format_rejects_unsupported`                                    |
| BR-005 | Output Filename Convention | UC-003 | Yes     | `uc003_build_output_path_*` (3)                                                                                           |
| BR-006 | Codec Optimization         | UC-003 | Yes     | `uc003_codec_for_format_*` (4)                                                                                            |
| BR-007 | Partial File Cleanup       | UC-004 | Partial | `uc004_cancel_flag_store_and_load` — flag tested, file deletion not                                                       |

## Summary

|            | Total | Tested | Partial | Not Tested |
|------------|-------|--------|---------|------------|
| Scenarios  | 14    | 6      | 1       | 7          |
| Bus. Rules | 7     | 5      | 2       | 0          |
| Tests      | 22    |        |         |            |

### Untested Scenarios — Justification

| Scenario      | Reason                                            |
|---------------|---------------------------------------------------|
| UC-001 A1, A2 | Frontend-only logic, no backend contract to test  |
| UC-002 A1, A2 | Frontend-only interaction / implicit in UC-003    |
| UC-003 A1     | Requires integration test (missing FFmpeg binary) |
| UC-003 A3     | Feature not yet implemented                       |
| UC-003 A4     | Frontend-only workflow repetition                 |
| UC-004 A1     | Race condition, non-deterministic                 |
