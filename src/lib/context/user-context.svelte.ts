// Set and get user context to determine if the user has permissions to see specific routes.

import { getContext, setContext } from 'svelte';
import type { AuthenticatedUser } from '$lib/db/adapters/auth';
import { authenticateUser, needsInitialSetup } from '$lib/db/adapters/auth';

const USER_CONTEXT_KEY = Symbol('user-context');

export interface UserContext {
	user: AuthenticatedUser | null;
	isAuthenticated: boolean;
	isInitialSetup: boolean;
	isLoading: boolean;
	hasPermission: (permission: string) => boolean;
	hasAnyPermission: (permissions: string[]) => boolean;
	hasAllPermissions: (permissions: string[]) => boolean;
	refresh: () => Promise<void>;
	setUser: (user: AuthenticatedUser | null) => void;
	setInitialSetup: (value: boolean) => void;
}

class UserContextImpl {
	user = $state<AuthenticatedUser | null>(null);
	isInitialSetup = $state(false);
	isLoading = $state(true);

	get isAuthenticated() {
		return this.user !== null;
	}

	hasPermission(permission: string): boolean {
		if (!this.user) return false;
		return this.user.permissions.includes(permission);
	}

	hasAnyPermission(permissions: string[]): boolean {
		if (!this.user) return false;
		return permissions.some((p) => this.user!.permissions.includes(p));
	}

	hasAllPermissions(permissions: string[]): boolean {
		if (!this.user) return false;
		return permissions.every((p) => this.user!.permissions.includes(p));
	}

	async refresh(): Promise<void> {
		this.isLoading = true;
		try {
			// Check if initial setup is needed
			const needsSetup = await needsInitialSetup();
			console.log('needsInitialSetup returned:', needsSetup);
			this.isInitialSetup = needsSetup;

			if (needsSetup) {
				this.user = null;
				console.log('Initial setup required - no users in database');
				return;
			}

			// Try to authenticate
			try {
				this.user = await authenticateUser();
				console.log('User authenticated successfully:', this.user);
			} catch (error) {
				console.error('Authentication error:', error);
				const errorMsg = String(error);
				if (errorMsg.includes('INITIAL_SETUP')) {
					this.isInitialSetup = true;
					this.user = null;
				} else if (errorMsg.includes('USER_NOT_REGISTERED')) {
					this.user = null;
					// User exists in AD but not in database
					console.warn('User not registered in system');
				} else {
					throw error;
				}
			}
		} finally {
			this.isLoading = false;
		}
	}

	setUser(user: AuthenticatedUser | null): void {
		this.user = user;
		this.isLoading = false;
	}

	setInitialSetup(value: boolean): void {
		this.isInitialSetup = value;
	}
}

export function createUserContext(): UserContext {
	return new UserContextImpl();
}

export function setUserContext(context: UserContext) {
	setContext(USER_CONTEXT_KEY, context);
}

export function getUserContext(): UserContext {
	return getContext<UserContext>(USER_CONTEXT_KEY);
}