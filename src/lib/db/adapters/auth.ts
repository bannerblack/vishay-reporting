import { invoke } from '@tauri-apps/api/core';
import type { UserData } from './user';

export interface AuthenticatedUser {
	id: number;
	username: string;
	name: string;
	permissions: string[];
	preferences: string;
}

export async function getSystemUser(): Promise<[string, string]> {
	try {
		return await invoke<[string, string]>('get_system_user');
	} catch (error) {
		throw new Error(`Failed to get system user: ${error}`);
	}
}

export async function authenticateUser(): Promise<AuthenticatedUser> {
	return await invoke<AuthenticatedUser>('authenticate_user');
}

export async function needsInitialSetup(): Promise<boolean> {
	try {
		return await invoke<boolean>('needs_initial_setup');
	} catch (error) {
		throw new Error(`Failed to check setup status: ${error}`);
	}
}

export async function validateAdminPassword(password: string): Promise<boolean> {
	try {
		return await invoke<boolean>('validate_admin_password', { password });
	} catch (error) {
		throw new Error(`Failed to validate password: ${error}`);
	}
}

export async function createInitialAdmin(
	password: string,
	userData: UserData
): Promise<void> {
	try {
		await invoke('create_initial_admin', { password, userData });
	} catch (error) {
		throw new Error(`Failed to create initial admin: ${error}`);
	}
}

export async function adminCreateUser(
	adminUsername: string,
	userData: UserData
): Promise<void> {
	try {
		await invoke('admin_create_user', { adminUsername, userData });
	} catch (error) {
		throw new Error(`Failed to create user: ${error}`);
	}
}

export async function userHasPermission(
	username: string,
	permission: string
): Promise<boolean> {
	try {
		return await invoke<boolean>('user_has_permission', { username, permission });
	} catch {
		return false;
	}
}
