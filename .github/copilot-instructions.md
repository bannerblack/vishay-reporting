# Vishay Testing Application - AI Agent Guide

## Project Overview
Tauri 2.9 desktop app for managing test specifications, finished goods (FG), and quality reports. Uses Rust backend (SeaORM + SQLite) with SvelteKit frontend. Windows-based Active Directory SSO authentication.

## Architecture Patterns

### Dual Database System
- **Core DB** (`testing.sqlite`): Users, FGs, Reports, Tests entities with SeaORM
- **Voltech DB** (`voltech.sqlite`): Parsed test result data from `.atr` files
- Connection setup: `src-tauri/src/lib.rs` → `establish_core_connection()` and `establish_voltech_connection()`
- State managed via `AppState` struct with `Arc<DbConn>` for thread-safe sharing

### Backend Module Pattern (Rust)
Each entity has dedicated module in `src-tauri/src/`:
- **Naming**: `<action>_<entity>` (e.g., `create_user`, `get_all_fgs`)
- **DTOs**: `<Entity>Data` (input), `<Entity>Response` (output)
- **Error handling**: All commands return `Result<T, String>` for Tauri serialization
- **Joins module**: Separate from CRUD - 13 relationship queries (e.g., `get_fg_with_reports`, `get_report_complete`)

Example command structure:
```rust
#[tauri::command]
async fn create_report(report_data: ReportData, state: State<'_, AppState>) -> Result<ReportResponse, String> {
    match operation {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to create report: {}", e))
    }
}
```

### Frontend Adapter Pattern (TypeScript)
All Tauri commands wrapped in `src/lib/db/adapters/`:
```typescript
// src/lib/db/adapters/report.ts
export async function createReport(reportData: ReportData): Promise<ReportResponse> {
    try {
        return await invoke<ReportResponse>('create_report', { reportData });
    } catch (error) {
        throw new Error(`Failed to create report: ${error}`);
    }
}
```
- **Import pattern**: `import { report, fg, test } from '$lib/db/database'` (re-exported from `database.ts`)
- Types mirror Rust DTOs exactly for type safety across IPC boundary

### Entity Relationships
```
FG (parent)
├── Reports (many) - fg_id FK
└── Tests (many) - fg_id FK

Report (child of FG)
└── Tests (many) - report_id FK (nullable for unassigned tests)
```
Critical: `report_id` nullable supports "test library" concept where tests exist before assignment to reports.

## Authentication System
**Windows SSO via `whoami` crate** - no passwords for end users
- Setup password: `vishay_admin_2025` (hardcoded in `src-tauri/src/auth.rs`)
- Initial setup flow: `/setup` → creates first admin with Windows username
- Permission model: `["admin", "pe", "qa"]` stored as JSON in User.preferences
- Context: `src/lib/context/user-context.svelte.ts` - Svelte 5 `$state` runes for reactive auth
- Route guards: `src/lib/config/routes.ts` - centralized permission checks

Check permissions in components:
```typescript
const userContext = getUserContext();
if (userContext.hasPermission('admin')) { /* ... */ }
```

## Database Migrations
SeaORM migrations in `src-tauri/migration/src/`:
- Auto-run on app startup via `Migrator::up()`
- **Important**: `Report.fg_id` is `integer` type (manually fixed in entity after generation)
- Multiple migration directories: `migration` (core), `migration_voltech` (test results)

## Key Development Workflows

### Adding New Tauri Command
1. Implement in relevant `src-tauri/src/<module>.rs`
2. Register in `lib.rs` invoke_handler (currently 43 commands)
3. Create TypeScript wrapper in `src/lib/db/adapters/<module>.ts`
4. Re-export from `src/lib/db/database.ts`

### Running the App
```powershell
npm run tauri dev    # Development mode (Vite + Tauri)
npm run tauri build  # Production build
```
Note: Dev mode slow due to Vite; production builds are optimized

### Testing
Testing will be implemented after database structures are finalized. For reference pattern, see `report_test.rs`:
```rust
#[tokio::test]
async fn test_name() {
    let db = setup_test_db().await;
    // Test operations
    cleanup_test_db(&db).await;
}
```
Run: `cd src-tauri ; cargo test`

## Voltech Integration (Work in Progress)
**Current State**: Standalone CLI in `src-tauri/voltech_parsing/` - needs SeaORM conversion

**System Purpose**: Parse Voltech test data from `.atr` files into database
- **File pattern**: `C#DDMMYY.atr` (CSV format, e.g., `C0181125.atr`)
- **Database**: Uses `entity_voltech` and `migration_voltech` (tables: `test_results`, `processed_files`, `settings`)
- **Watch mode**: Real-time monitoring (polls every 10s) + weekly 30-day scan
- **Import mode**: Bulk historical data import

**Integration Roadmap**:
1. Convert file watcher to Tauri background task
2. Implement Tauri commands for watch/import operations
3. Add frontend UI for voltech data queries
4. Use existing `establish_voltech_connection()` in `lib.rs`

**Key Files**:
- `voltech_parsing/src/file_watcher.rs` - File monitoring logic to integrate
- `voltech_parsing/src/parser.rs` - CSV parsing functions to preserve
- `entity_voltech/src/` - Already defined SeaORM entities
- See `voltech_parsing/PROCESS_OUTLINE.md` for detailed system design

## UI Component Library
**shadcn-svelte** components in `src/lib/components/ui/`:
- Full suite: buttons, tables, forms, dialogs, sidebars
- Import: `import * as Dialog from "$lib/components/ui/dialog"` 
- Styling: Tailwind CSS 4.x with custom theme

## SvelteKit Specifics
- **Adapter**: `@sveltejs/adapter-static` (required for Tauri)
- **CSP**: Disabled for Tauri compatibility (`csp: null`)
- **Routing**: File-based in `src/routes/` - `+page.svelte`, `+layout.svelte` pattern
- **Context**: Svelte 5 uses `$state` runes, not stores (see user-context for pattern)

## Tauri Command Pattern
Every backend function follows this structure:

**1. Define DTOs in module file:**
```rust
// Input DTO
#[derive(Debug, Deserialize)]
pub struct ReportData {
    pub fg_id: i32,
    pub attributes: String,
    pub added_by: i32,
}

// Output DTO
#[derive(Debug, Serialize)]
pub struct ReportResponse {
    pub id: i32,
    pub fg_id: i32,
    pub attributes: String,
    pub added_by: i32,
    pub created_at: String,
    pub updated_at: String,
}
```

**2. Implement business logic function:**
```rust
pub async fn create_report(db: &DbConn, report_data: ReportData) -> Result<ReportResponse, DbErr> {
    let new_report = report::ActiveModel {
        fg_id: Set(report_data.fg_id),
        attributes: Set(report_data.attributes),
        added_by: Set(report_data.added_by),
        ..Default::default()
    };
    
    let result = new_report.insert(db).await?;
    Ok(ReportResponse {
        id: result.id,
        fg_id: result.fg_id,
        // ... map fields
    })
}
```

**3. Create Tauri command wrapper:**
```rust
#[tauri::command]
async fn create_report(
    report_data: ReportData,
    state: State<'_, AppState>
) -> Result<ReportResponse, String> {
    match report::create_report(&state.core_db, report_data).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to create report: {}", e))
    }
}
```

**4. Register in lib.rs invoke_handler:**
```rust
.invoke_handler(tauri::generate_handler![
    create_report,
    get_report,
    // ... other commands
])
```

## Database CRUD Patterns

### Standard CRUD Operations
All entity modules implement these 6 operations:
```rust
// CREATE
pub async fn create_entity(db: &DbConn, data: EntityData) -> Result<EntityResponse, DbErr>

// READ (single)
pub async fn get_entity(db: &DbConn, id: i32) -> Result<EntityResponse, DbErr>

// READ (all)
pub async fn get_all_entities(db: &DbConn) -> Result<Vec<EntityResponse>, DbErr>

// UPDATE
pub async fn update_entity(db: &DbConn, id: i32, data: EntityData) -> Result<EntityResponse, DbErr>

// DELETE
pub async fn delete_entity(db: &DbConn, id: i32) -> Result<String, DbErr>

// FIND BY UNIQUE FIELD (optional)
pub async fn get_entity_by_field(db: &DbConn, field: String) -> Result<EntityResponse, DbErr>
```

### Entity Model to Response Conversion
Consistent pattern for mapping SeaORM models to DTOs:
```rust
fn model_to_response(model: entity::Model) -> EntityResponse {
    EntityResponse {
        id: model.id,
        field: model.field,
        created_at: model.created_at.to_string(),
        updated_at: model.updated_at.to_string(),
    }
}
```

### Update Pattern with Partial Updates
```rust
pub async fn update_entity(db: &DbConn, id: i32, data: EntityData) -> Result<EntityResponse, DbErr> {
    let entity = Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Entity not found".to_string()))?;
    
    let mut active: entity::ActiveModel = entity.into();
    active.field = Set(data.field);
    active.updated_at = Set(chrono::Utc::now().naive_utc());
    
    let updated = active.update(db).await?;
    Ok(model_to_response(updated))
}
```

## Svelte 5 Context Pattern (Reactive State)

**Creating Context (`src/lib/context/user-context.svelte.ts`):**
```typescript
import { getContext, setContext } from 'svelte';

class UserContext {
    user = $state<AuthenticatedUser | null>(null);
    isLoading = $state(true);
    isAuthenticated = $state(false);

    async refresh() {
        this.isLoading = true;
        try {
            this.user = await auth.authenticateUser();
            this.isAuthenticated = true;
        } catch (error) {
            this.user = null;
            this.isAuthenticated = false;
        } finally {
            this.isLoading = false;
        }
    }

    hasPermission(permission: string): boolean {
        return this.user?.permissions?.includes(permission) ?? false;
    }
}

const USER_CONTEXT_KEY = Symbol('user');

export function createUserContext() {
    return new UserContext();
}

export function setUserContext(context: UserContext) {
    setContext(USER_CONTEXT_KEY, context);
}

export function getUserContext(): UserContext {
    return getContext<UserContext>(USER_CONTEXT_KEY);
}
```

**Using in Layout (`+layout.svelte`):**
```svelte
<script lang="ts">
    import { createUserContext, setUserContext } from '$lib/context/user-context.svelte';
    import { onMount } from 'svelte';

    const userContext = createUserContext();
    setUserContext(userContext);

    onMount(async () => {
        await userContext.refresh();
    });
</script>

{#if userContext.isLoading}
    <div>Loading...</div>
{:else if userContext.isAuthenticated}
    {@render children()}
{/if}
```

**Consuming in Components:**
```svelte
<script lang="ts">
    import { getUserContext } from '$lib/context/user-context.svelte';
    
    const userContext = getUserContext();
    
    // Reactive derived values in Svelte 5
    const canEdit = $derived(userContext.hasPermission('admin'));
</script>

{#if canEdit}
    <button>Edit</button>
{/if}
```

## Svelte 5 Runes Reference
- `$state()` - reactive state (replaces writable stores)
- `$derived()` - computed values (replaces derived stores)
- `$effect()` - side effects (replaces reactive statements)
- `$props()` - component props with destructuring
- `$bindable()` - two-way binding for component props

## Common Pitfalls
1. **Report.fg_id type**: Entity generator creates as String, manually change to i32
2. **Command naming conflicts**: `auth::get_user` renamed to `auth::get_system_user` to avoid clash with `user::get_user`
3. **Database paths**: Temporarily hardcoded to `C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/*.sqlite` in lib.rs - production path strategy TBD
4. **Tauri dev server**: CSRF origins configured for `tauri://localhost` protocol
5. **Nullable foreign keys**: Test.report_id must support null in both schema and business logic
6. **Svelte 5 migration**: Use runes (`$state`, `$derived`) not stores - context pattern differs from Svelte 4

## File Locations Reference
- **Entities**: `src-tauri/entity/src/` (SeaORM generated, sometimes needs manual fixes)
- **Entities Voltech**: `src-tauri/entity_voltech/src/` (SeaORM generated, sometimes needs manual fixes)
- **Migrations**: `src-tauri/migration/src/` (SeaORM generated, sometimes needs manual fixes)
- **Migration Voltech**: `src-tauri/migration_voltech/src/` (SeaORM generated, sometimes needs manual fixes)
- **Documentation**: `ARCHITECTURE.md`, `AUTHENTICATION.md` (comprehensive system docs)
- **Config**: `tauri.conf.json`, `svelte.config.js`, `drizzle.config.ts`
- **Types**: Shared types in `src/types/types.ts` (though most use adapter types)
