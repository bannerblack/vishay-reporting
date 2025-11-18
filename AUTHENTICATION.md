# Authentication System Implementation

## Overview
Complete Active Directory-based authentication system with database-backed permissions, initial setup flow, and role-based access control.

## Features Implemented

### Backend (Rust)

#### Enhanced `auth.rs`
- **`authenticate_user()`**: Authenticates current Windows user against database
- **`needs_initial_setup()`**: Checks if any users exist in database
- **`validate_admin_password()`**: Validates hardcoded admin setup password
- **`create_initial_admin()`**: Creates first admin with password validation
- **`admin_create_user()`**: Admin-only user creation with permission check
- **`user_has_permission()`**: Check if user has specific permission
- **Hardcoded Setup Password**: `vishay_admin_2025`

#### Updated `lib.rs`
- Registered all new authentication commands in invoke_handler

### Frontend (TypeScript/Svelte)

#### Auth Adapter (`src/lib/db/adapters/auth.ts`)
- Complete TypeScript bindings for all auth commands
- Type-safe interfaces for `AuthenticatedUser`
- Error handling preserves backend error codes

#### User Context (`src/lib/context/user-context.svelte.ts`)
- Reactive Svelte 5 context using `$state` runes
- Methods:
  - `hasPermission(permission)`: Check single permission
  - `hasAnyPermission(permissions[])`: Check if user has ANY
  - `hasAllPermissions(permissions[])`: Check if user has ALL
  - `refresh()`: Re-authenticate user from backend
- Handles initial setup and unregistered user states

#### Route Config (`src/lib/config/routes.ts`)
- Centralized permission configuration
- Routes:
  - `/`, `/locked`, `/setup`: Public
  - `/manage/*`: Requires `pe` OR `qa`
  - `/manage/user`: Requires `admin`
- `canAccessRoute()` utility function

#### Setup Page (`src/routes/setup/+page.svelte`)
- Displays current Windows user info
- Admin password input with validation
- Auto-creates first admin with full permissions (`admin`, `pe`, `qa`)
- Redirects to home after successful setup

#### Updated Layout (`src/routes/+layout.svelte`)
- Initializes user context on mount
- Shows loading state during authentication
- Redirects to setup if needed
- Shows access denied for unregistered users
- Only renders app UI for authenticated users

#### Updated Sidebar (`src/lib/components/app-sidebar.svelte`)
- Dynamic navigation based on permissions
- Manage section hidden without `pe`/`qa`/`admin`
- Users menu item hidden without `admin`
- Displays authenticated user info

## Authentication Flow

### Initial Setup (No Users)
1. App loads → detects no users → redirects to `/setup`
2. User enters admin password: `vishay_admin_2025`
3. System creates user with their Windows credentials + admin permissions
4. User redirected to home, fully authenticated

### Normal Authentication (Users Exist)
1. App loads → `whoami` gets Windows username
2. Looks up username in database
3. If found: Load user with permissions → render app
4. If not found: Show "Access Denied" message

### Adding New Users
- Admin navigates to `/manage/user`
- Creates new user (auto-captures their Windows username)
- Assigns permissions via JSON array: `["pe", "qa"]`
- User can now log in automatically via Windows SSO

## Permission System

### Available Permissions
- `admin`: Full system access, can manage users
- `pe`: Product Engineering, can manage FGs/reports/tests  
- `qa`: Quality Assurance, can manage FGs/reports/tests

### Permission Checking
```typescript
// In components
const userContext = getUserContext();
const canEdit = userContext.hasPermission('admin');
const canManage = userContext.hasAnyPermission(['pe', 'qa']);
```

### Route Protection
Configured in `src/lib/config/routes.ts`:
```typescript
{
  path: '/manage',
  permissions: ['pe', 'qa'],
  requireAll: false  // ANY permission
}
```

## Admin Password

**Default**: `vishay_admin_2025`

**To Change**: Edit `ADMIN_SETUP_PASSWORD` in `src-tauri/src/auth.rs`

**Production**: Consider environment variable or deployment-specific config

## Database Schema

Users table now includes:
- `permissions`: JSON array of permission strings
  Example: `["admin", "pe", "qa"]`

## Testing

1. **Initial Setup**: Delete database, restart app, use password
2. **Add User**: Login as admin → Manage → Users → Create
3. **Permission Check**: Login as non-admin, verify restricted access
4. **Multiple Permissions**: Create user with `["pe", "qa"]`, verify both work

## Enterprise Recommendations

1. **Environment-based password**: Store in `.env` or deployment config
2. **AD Group sync**: Future enhancement to sync permissions from AD groups
3. **Audit logging**: Log permission changes and user creation
4. **Password rotation**: Change setup password after initial deployment
5. **Multi-admin setup**: Ensure multiple admins during initial setup

## Changeover Strategy

1. Deploy with setup password shared with IT
2. IT runs setup, creates first admin
3. First admin creates other admins
4. First admin creates all users with appropriate permissions
5. Users auto-authenticate via Windows SSO
6. No disruption - users just need to be added to database once
