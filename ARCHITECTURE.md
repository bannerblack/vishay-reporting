# Vishay Testing Application Architecture

## Overview
Full-stack desktop application built with Tauri 2.9, SvelteKit, and SeaORM for managing test reports, finished goods (FG), and test specifications.

## Tech Stack
- **Backend**: Rust, Tauri 2.9, SeaORM 1.1, SQLite
- **Frontend**: SvelteKit, TypeScript, Vite 7.x
- **Database**: SQLite with SeaORM migrations

## Database Schema

### Tables
```
User (standalone)
├── id (Primary Key)
├── name
├── username
├── preferences
└── timestamps

FG (parent entity)
├── id (Primary Key)
├── fg (Finished Good number)
├── rev (Revision)
└── customer

Report (child of FG, parent of Test)
├── id (Primary Key)
├── fg_id (Foreign Key → FG.id)
├── attributes
├── added_by (Foreign Key → User.id)
└── timestamps

Test (child of FG and Report)
├── id (Primary Key)
├── fg_id (Foreign Key → FG.id)
├── report_id (Foreign Key → Report.id, nullable)
├── test_type
├── frequency, voltage, min, max
├── uo_m (Unit of Measure)
├── primary_pins, secondary_pins, shorted_pins
├── description
├── added_by (Foreign Key → User.id)
└── timestamps
```

### Relationships
- FG has many Reports
- FG has many Tests
- Report belongs to FG
- Report has many Tests
- Test belongs to FG (required)
- Test belongs to Report (optional - unassigned tests allowed)

## Backend Architecture

### Module Structure
```
src-tauri/src/
├── lib.rs              # Main entry, module registration, 43 Tauri commands
├── database.rs         # Shared database models
├── auth.rs             # System user authentication
├── user.rs             # User CRUD (6 operations)
├── fg.rs               # FG CRUD (6 operations)
├── report.rs           # Report CRUD (5 operations)
├── test.rs             # Test CRUD + assignment (7 operations)
└── joins.rs            # Relationship queries (13 operations)
```

### Naming Conventions
- **Commands**: `<action>_<entity>` (e.g., `create_user`, `get_all_fgs`)
- **Input DTOs**: `<Entity>Data` (e.g., `UserData`, `FGData`)
- **Output DTOs**: `<Entity>Response` (e.g., `UserResponse`, `FGResponse`)
- **Join DTOs**: `<Entity>With<Relation>Response` (e.g., `FGWithReportsResponse`)

### Error Handling Pattern
All CRUD functions return `Result<T, String>` for serializable errors:
```rust
match operation {
    Ok(result) => Ok(result),
    Err(e) => Err(format!("Failed to <action> <entity>: {}", e))
}
```

### Key Features

#### User Module (`user.rs`)
- `create_user(UserData) → UserResponse`
- `get_user(id) → UserResponse`
- `get_user_by_username(username) → UserResponse`
- `get_all_users() → Vec<UserResponse>`
- `update_user(id, UserData) → UserResponse`
- `delete_user(id) → String`

#### FG Module (`fg.rs`)
- Duplicate checking on create/update by FG number
- `create_fg(FGData) → FGResponse`
- `get_fg(id) → FGResponse`
- `get_fg_by_number(fg_number) → FGResponse`
- `get_all_fgs() → Vec<FGResponse>`
- `update_fg(id, FGData) → FGResponse`
- `delete_fg(id) → String`

#### Report Module (`report.rs`)
- Foreign key validation (fg_id references FG.id as integer)
- `create_report(ReportData) → ReportResponse`
- `get_report(id) → ReportResponse`
- `get_all_reports() → Vec<ReportResponse>`
- `update_report(id, ReportData) → ReportResponse`
- `delete_report(id) → String`

#### Test Module (`test.rs`)
- Support for unassigned tests (report_id nullable)
- Comprehensive test parameters (frequency, voltage, pins, etc.)
- `create_test(TestData) → TestResponse`
- `get_test(id) → TestResponse`
- `get_all_tests() → Vec<TestResponse>`
- `update_test(id, TestData) → TestResponse`
- `delete_test(id) → String`
- `assign_test_to_report(test_id, report_id) → TestResponse`
- `unassign_test_from_report(test_id) → TestResponse`

#### Joins Module (`joins.rs`)
13 relationship query operations:

**FG Joins:**
- `get_fg_with_reports(fg_id) → FGWithReportsResponse`
- `get_fg_with_tests(fg_id) → FGWithTestsResponse`
- `get_fg_complete(fg_id) → FGCompleteResponse` (all relations)

**Report Joins:**
- `get_report_with_fg(report_id) → ReportWithFGResponse`
- `get_report_with_tests(report_id) → ReportWithTestsResponse`
- `get_report_complete(report_id) → ReportCompleteResponse`

**Test Joins:**
- `get_test_with_fg(test_id) → TestWithFGResponse`
- `get_test_with_report(test_id) → TestWithReportResponse`
- `get_test_complete(test_id) → TestCompleteResponse`

**List Operations:**
- `get_all_reports_with_fg() → Vec<ReportWithFGResponse>`
- `get_all_tests_by_fg(fg_id) → Vec<TestResponse>`
- `get_all_tests_by_report(report_id) → Vec<TestResponse>`
- `get_available_tests_for_report(report_id) → Vec<TestResponse>` (unassigned or from same FG)

## Frontend Architecture

### Adapter Layer
```
src/lib/db/
├── database.ts         # Main orchestrator, exports all modules
└── adapters/
    ├── user.ts         # User CRUD wrappers
    ├── fg.ts           # FG CRUD wrappers
    ├── report.ts       # Report CRUD wrappers
    ├── test.ts         # Test CRUD wrappers
    └── joins.ts        # Join operation wrappers
```

### Adapter Pattern
All adapters wrap Tauri `invoke()` with:
- TypeScript interfaces matching Rust DTOs
- Error handling and type safety
- Clean async/await API

Example:
```typescript
export async function createUser(userData: UserData): Promise<UserResponse> {
    try {
        return await invoke<UserResponse>('create_user', { userData });
    } catch (error) {
        throw new Error(`Failed to create user: ${error}`);
    }
}
```

## Database Migrations

### Migration System
- SeaORM migrations in `src-tauri/migration/`
- Auto-run on connection establishment
- Current migration: `m20220101_000001_create_table.rs`

### Schema Notes
- Report.fg_id: **integer** (matches FG.id type)
- All foreign keys: CASCADE on update/delete
- Test.report_id: nullable for unassigned tests
- Timestamps: created_at, updated_at on Report and Test

## Testing

### Current Test Coverage
- `report_test.rs`: 9 comprehensive tests covering full CRUD lifecycle

### Test Pattern
```rust
#[tokio::test]
async fn test_name() {
    let db = setup_test_db().await;
    // Test operations
    cleanup_test_db(&db).await;
}
```

## Command Registration

All 43 commands registered in `lib.rs`:
- 2 Authentication commands
- 6 User CRUD commands
- 6 FG CRUD commands
- 5 Report CRUD commands
- 7 Test CRUD commands (including assign/unassign)
- 13 Join operation commands
- 4 Other commands (sheets, database, voltech)

## Build & Run

### Development
```powershell
# Backend
cd src-tauri
cargo build

# Frontend + Backend
npm run tauri dev

# Tests
cargo test
```

### Production
```powershell
npm run tauri build
```

## Key Design Decisions

1. **Modular Structure**: Separate files per entity for maintainability
2. **Joins Module**: Centralized relationship queries separate from CRUD
3. **Nullable report_id**: Supports test library concept (unassigned tests)
4. **Type Safety**: Integer foreign keys, proper entity references
5. **Frontend Adapters**: Clean TypeScript layer over Tauri IPC
6. **Error Serialization**: String errors for cross-language compatibility
7. **Duplicate Prevention**: FG number uniqueness checks in business logic

## Future Enhancements

### Recommended Next Steps
1. Add test files: `user_test.rs`, `fg_test.rs`, `test_test.rs`
2. Implement UI components using frontend adapters
3. Add pagination for large list operations
4. Consider caching frequently accessed data
5. Add batch operations for bulk imports
6. Implement audit logging for data changes

## Notes

- **Command Conflict**: Renamed `auth::get_user` to `auth::get_system_user` to avoid conflict with `user::get_user`
- **Entity Generation**: Manual fix for Report.fg_id type (String→i32) in entity file
- **Performance**: SvelteKit in dev mode may be slow, production builds optimized
- **Database**: SQLite file created on first run via migrations
