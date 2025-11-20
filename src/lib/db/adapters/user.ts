import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types
// ============================================================================

export interface UserData {
	name: string;
	username: string;
	preferences: string;
	permissions: string;
	added_by?: number | null;
}

export interface UserResponse {
	id: number;
	name: string;
	username: string;
	preferences: string;
	permissions: string;
	created_at: string;
	updated_at: string;
}

export interface UserPreferences {
	theme: string;        // "light", "dark", "blue", "green", "purple", "system"
	language: string;     // "en", "es", "de", etc.
	notifications: boolean;
}

export interface UpdatePreferencesData {
	preferences: UserPreferences;
}

// ============================================================================
// Adapter Functions
// ============================================================================

export async function createUser(userData: UserData): Promise<UserResponse> {
	try {
		return await invoke<UserResponse>('create_user', { userData });
	} catch (error) {
		throw new Error(`Failed to create user: ${error}`);
	}
}

export async function getUser(id: number): Promise<UserResponse> {
	try {
		return await invoke<UserResponse>('get_user', { id });
	} catch (error) {
		throw new Error(`Failed to get user: ${error}`);
	}
}

export async function getUserByUsername(username: string): Promise<UserResponse> {
	try {
		return await invoke<UserResponse>('get_user_by_username', { username });
	} catch (error) {
		throw new Error(`Failed to get user by username: ${error}`);
	}
}

export async function getAllUsers(): Promise<UserResponse[]> {
	try {
		return await invoke<UserResponse[]>('get_all_users');
	} catch (error) {
		throw new Error(`Failed to get all users: ${error}`);
	}
}

export async function updateUser(id: number, userData: UserData): Promise<UserResponse> {
	try {
		return await invoke<UserResponse>('update_user', { id, userData });
	} catch (error) {
		throw new Error(`Failed to update user: ${error}`);
	}
}

export async function deleteUser(id: number): Promise<string> {
	try {
		return await invoke<string>('delete_user', { id });
	} catch (error) {
		throw new Error(`Failed to delete user: ${error}`);
	}
}

// ============================================================================
// Preferences Functions
// ============================================================================

export async function getUserPreferences(userId: number): Promise<UserPreferences> {
	try {
		return await invoke<UserPreferences>('get_user_preferences', { userId });
	} catch (error) {
		throw new Error(`Failed to get user preferences: ${error}`);
	}
}

export async function updateUserPreferences(
	userId: number,
	preferencesData: UpdatePreferencesData
): Promise<UserResponse> {
	try {
		return await invoke<UserResponse>('update_user_preferences', {
			userId,
			preferencesData
		});
	} catch (error) {
		throw new Error(`Failed to update user preferences: ${error}`);
	}
}
