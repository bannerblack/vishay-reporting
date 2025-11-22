 
# Types & Structs Map

This document maps the application's domain types across Rust (SeaORM entities and Tauri command modules) and TypeScript (types and adapter functions). It is intended as a quick reference for where each type/struct is defined and which modules expose methods that operate on them.

## Table of contents

- [[#Test]]
- [[#Report]]
- [[#Finished Good (FG)]]
- [[#User]]
- [[#Test Types / Associated Tests]]
- [[#Voltech (parsed test results)]]
- [[#Manual test results]]
- [[#Joins and aggregations]]
- [[#Frontend adapters & types]]

---

## Test

- **SeaORM entity (Rust):** `src-tauri/entity/src/test.rs`
- **SeaORM properties:**
```rust
id: i32
report_id: Option<i32>
fg_id: i32
test_type: String
frequency: Option<f64>
voltage: Option<f64>
minimum: Option<f64>
maximum: Option<f64>
uo_m: String
primary_pins: Option<String>
secondary_pins: Option<String>
shorted_pins: Option<String>
description: Option<String>
added_by: Option<i32>
created_at: DateTime
updated_at: DateTime
order: i32
source_type: String
associated_test: Option<String>
manual_override: Option<bool>
```

- **Rust DTOs (module):** `src-tauri/src/test.rs` — `TestData` (input) and `TestResponse` (output).
- **Rust commands (Tauri):** `create_test`, `get_test`, `get_all_tests`, `update_test`, `delete_test`, `assign_test_to_report`, `unassign_test_from_report`, `update_test_order` (all in `src-tauri/src/test.rs`).
- **Join helpers (Rust):** `src-tauri/src/joins.rs` contains composite responses and commands such as `get_test_with_fg`, `get_test_with_report`, `get_test_complete`, `get_all_tests_by_fg`, `get_all_tests_by_report`, `get_available_tests_for_report`.
- **TypeScript (types):** `TestData` and `TestResponse` interfaces in `src/lib/db/adapters/test.ts`. Frontend validation types: `eTest` and `eTestSchema` in `src/types/types.ts`.
- **TypeScript (adapters):** `src/lib/db/adapters/test.ts` exports `createTest`, `getTest`, `getAllTests`, `updateTest`, `deleteTest`, `assignTestToReport`, `unassignTestFromReport`, `updateTestOrder`.

---

## Report

- **SeaORM entity (Rust):** `src-tauri/entity/src/report.rs`
- **SeaORM properties:**
```rust
id: i32
fg_id: i32
attributes: String
added_by: Option<i32>
created_at: DateTime
updated_at: DateTime
```

- **Rust DTOs (module):** `src-tauri/src/report.rs` — `ReportData`, `ReportResponse`.
- **Rust commands (Tauri):** `create_report`, `get_report`, `get_all_reports`, `update_report`, `delete_report`.
- **Join commands (Rust):** `get_report_with_fg`, `get_report_with_tests`, `get_report_complete`, `get_all_reports_with_fg` (in `src-tauri/src/joins.rs`).
- **TypeScript (types):** `ReportData`, `ReportResponse` in `src/lib/db/adapters/report.ts`.
- **TypeScript (adapters):** `createReport`, `getReport`, `getAllReports`, `updateReport`, `deleteReport` in `src/lib/db/adapters/report.ts`.

---

## Finished Good (FG)

- **SeaORM entity (Rust):** `src-tauri/entity/src/fg.rs`
- **SeaORM properties:**
```rust
id: i32
fg: String
rev: String
customer: String
serialized: bool
```

- **Rust DTOs (module):** `src-tauri/src/fg.rs` — `FGData`, `FGResponse`.
- **Rust commands (Tauri):** `create_fg`, `get_fg`, `get_fg_by_number`, `get_all_fgs`, `update_fg`, `delete_fg`.
- **Join commands (Rust):** `get_fg_with_reports`, `get_fg_with_tests`, `get_fg_complete` in `src-tauri/src/joins.rs`.
- **TypeScript (types):** `FGData`, `FGResponse` in `src/lib/db/adapters/fg.ts`. Frontend schema: `newFgSchema` in `src/types/types.ts`.
- **TypeScript (adapters):** `createFG`, `getFG`, `getFGByNumber`, `getAllFGs`, `updateFG`, `deleteFG` in `src/lib/db/adapters/fg.ts`.

---

## User

- **SeaORM entity (Rust):** `src-tauri/entity/src/user.rs`
- **SeaORM properties:**
```rust
id: i32
name: String
username: String
preferences: String
added_by: Option<i32>
created_at: DateTime
updated_at: DateTime
permissions: String
```

- **Rust DTOs (module):** `src-tauri/src/user.rs` — `UserData`, `UserResponse`, `UserPreferences`, `UpdatePreferencesData`.
- **Rust commands / logic:** `create_user_logic`, `create_user`, `get_user`, `get_user_by_username`, `get_all_users`, `update_user`, `delete_user`, `get_user_preferences`, `update_user_preferences`.
- **Authentication (Rust):** `src-tauri/src/auth.rs` — `AuthenticatedUser`, `AdminSetupRequest`, commands: `get_system_user`, `authenticate_user`, `needs_initial_setup`, `validate_admin_password`, `create_initial_admin`, `admin_create_user`, `user_has_permission`, `get_user_roles`.
- **TypeScript (types):** `User`, `winUser` in `src/types/types.ts` and adapter interfaces in `src/lib/db/adapters/user.ts` (`UserData`, `UserResponse`, `UserPreferences`, `UpdatePreferencesData`).
- **TypeScript (adapters):** `createUser`, `getUser`, `getUserByUsername`, `getAllUsers`, `updateUser`, `deleteUser`, `getUserPreferences`, `updateUserPreferences` in `src/lib/db/adapters/user.ts`.

---

## Test Types / Associated Tests

- **Rust (definition & logic):** `src-tauri/src/test_types.rs` — defines `TestType`, `TEST_TYPES`, `AssociatedTests`, and functions `get_test_type_options`, `find_associated_tests`.
- **Data sources:** `find_associated_tests` queries both `entity_voltech::test_results` and `entity_manual::manual_test_results`.
- **TypeScript (types):** `AssociatedTests` interface in `src/lib/db/adapters/test-types.ts`.
- **TypeScript (adapters):** `getTestTypes`, `findTestsForType` in `src/lib/db/adapters/test-types.ts`.

---

## Voltech (parsed test results)

- **SeaORM entity (Rust):** `src-tauri/entity_voltech/src/test_results.rs`
- **SeaORM properties:**
```rust
id: i32
part: String
operator: String
batch: String
date: String
serial_num: String
result_num: i32
pass_fail: String
time: Option<String>
retries: Option<String>
file_path: String
measurements: String (Text)
created_at: DateTimeWithTimeZone
normalized_date: Option<Date>
```

- **Other voltech entities:** `processed_files.rs`, `settings.rs`, `watcher_lock.rs` under `src-tauri/entity_voltech/src/`.
**TypeScript (adapters):** `src/lib/db/adapters/voltech.ts` — functions for import/watch and queries against voltech data.

- Watcher / control:
  - `startVoltechWatcher`, `stopVoltechWatcher`, `pauseVoltechWatcher`, `resumeVoltechWatcher`, `getVoltechWatcherStatus`, `forceAcquireVoltechMaster`

- Settings:
  - `getVoltechSettings`, `setVoltechSetting`, `getAllVoltechSettings`, `deleteVoltechSetting`, `updateServerPathSetting`

- Errors:
  - `getVoltechErrors`, `acknowledgeVoltechErrors`, `acknowledgeFileErrors`, `clearAcknowledgedVoltechErrors`

- Lock:
  - `getVoltechLockStatus`, `forceReleaseVoltechLock`

- Batch queries:
  - `getRecentBatchesForPart`, `getBatchDetails`, `getBatchTests`, `getBatchesForPart`, `searchBatches`

- Part queries:
  - `getAllParts`, `getPartSummary`, `searchParts`, `getAllPartNumbers`, `getPartStatsByDate`

- Test queries:
  - `searchTests`, `getTestsBySerial`, `getFailedTests`, `getTestById`, `getTestsByBatch`, `countTests`, `getRecentTests`

- Stats:
  - `getDailyStats`, `getOperatorStats`, `getOverallStats`, `getPartStats`

- Full import / maintenance:
  - `resetVoltechDatabase`, `fullImportVoltechFiles`, `importVoltechFiles`, `runVoltechMaintenanceScan`

- Event listeners:
  - `onVoltechWatcherPaused`, `onVoltechWatcherResumed`, `onVoltechBatchProgress`, `onVoltechMaintenanceStart`, `onVoltechMaintenanceProgress`, `onVoltechMaintenanceComplete`

**Backend (Rust) — Tauri commands (voltech_parsing + `src-tauri/src/voltech/commands.rs`):** grouped by purpose

- Watcher / control:
  - `start_voltech_watcher`, `stop_voltech_watcher`, `pause_voltech_watcher`, `resume_voltech_watcher`, `get_voltech_watcher_status`, `force_acquire_voltech_master`

- Settings:
  - `get_voltech_settings`, `set_voltech_setting`, `get_all_voltech_settings`, `delete_voltech_setting`, `update_server_path_setting`

- Errors:
  - `get_voltech_errors`, `acknowledge_voltech_errors`, `acknowledge_file_errors`, `cleanup_old_voltech_errors`

- Lock:
  - `get_voltech_lock_status`, `force_release_voltech_lock`

- Batches:
  - `get_recent_batches_for_part`, `get_batch_details`, `get_batch_tests`, `get_batch_tests_raw`, `get_part_batches`, `search_batches`

- Parts:
  - `search_parts`, `get_all_parts`, `get_part_statistics` / `get_part_summary`, `get_part_daily_statistics`, `get_part_serial_numbers`, `get_parts_by_date_range`

- Tests:
  - `search_tests`, `search_tests_by_serial`, `search_tests_by_serial_range`, `get_failed_tests`, `get_tests_by_date_range`, `get_tests_by_file`, `get_test_by_serial`, `get_tests_by_batch`

- Stats:
  - `get_global_statistics`, `get_daily_statistics`, `get_operator_statistics`, `get_top_failed_parts`, `get_recent_activity`, `get_trend_data`, `get_daily_stats`, `get_operator_stats`, `get_overall_stats`, `get_part_stats`

- Full import / maintenance:
  - `import_voltech_files`, `full_import_voltech_files`, `reset_voltech_database`, `run_voltech_maintenance_scan`

- Events (emitted):
  - `voltech-batch-progress`, `voltech-maintenance-start`, `voltech-maintenance-progress`, `voltech-maintenance-complete`

---

## Manual test results

- **SeaORM entity crate:** `src-tauri/entity_manual/` (referenced as `entity_manual` in Rust code).
- **Usage:** referenced by `src-tauri/src/test_types.rs` and `voltech_parsing` for searching manual test names and results.

**Backend (Rust) — Tauri commands for manual tests (`src-tauri/src/manual/commands.rs`):**

- Import operations:
  - `import_manual_file`, `import_manual_fg_folder`

- Queries / summaries:
  - `get_manual_test_names`, `get_manual_tests`, `get_manual_summary`

- Settings / paths:
  - `get_manual_base_path`, `set_manual_base_path`

- **Query & operations modules:** `src-tauri/src/manual/queries.rs` and `src-tauri/src/manual/operations.rs` implement filters, imports, and helper logic used by the commands above.

---

## Joins and aggregations

- **Rust module:** `src-tauri/src/joins.rs` — defines composite response DTOs (e.g., `FGWithReportsResponse`, `TestCompleteResponse`) and many Tauri commands that return those composed shapes.
- **Pattern:** these functions fetch SeaORM `Entity` models and map them to small response DTOs defined in the same file.

---

## Frontend adapters & types

- **Adapter index:** `src/lib/db/database.ts` (re-exports all adapters).
- **Adapter files:** `src/lib/db/adapters/*.ts` — each adapter mirrors Rust DTOs with TypeScript interfaces and exposes functions that call `invoke('<tauri_command>')`.
- **Shared frontend types:** `src/types/types.ts` — validation schemas and small utility types used in UI forms.

---

## Notes & next steps

- The SeaORM entity files under `src-tauri/entity/src` and `src-tauri/entity_voltech/src` are the authoritative schema sources (column types & nullability).
- I can produce a machine-readable manifest (JSON or YAML) that lists each entity field and its type if you want to integrate this into tools or docs.
- I can also expand entries with exact method signatures or copy-paste code snippets for DTO definitions.

---

## Quick reference — last scanned files

Paths I scanned while creating this map:

- `src-tauri/src/test.rs`
- `src-tauri/src/report.rs`
- `src-tauri/src/fg.rs`
- `src-tauri/src/user.rs`
- `src-tauri/src/test_types.rs`
- `src-tauri/src/joins.rs`
- `src-tauri/src/auth.rs`
- `src-tauri/entity/src/test.rs`
- `src-tauri/entity/src/report.rs`
- `src-tauri/entity/src/fg.rs`
- `src-tauri/entity/src/user.rs`
- `src-tauri/entity_voltech/src/test_results.rs`
- `src/lib/db/adapters/test.ts`
- `src/lib/db/adapters/report.ts`
- `src/lib/db/adapters/fg.ts`
- `src/lib/db/adapters/user.ts`
- `src/lib/db/adapters/test-types.ts`
- `src/types/types.ts`

