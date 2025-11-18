import { redirect } from '@sveltejs/kit';
import type { LoadEvent } from '@sveltejs/kit';
import { canAccessRoute } from './routes';
import { authenticateUser, needsInitialSetup } from '$lib/db/adapters/auth';

export async function requireAuth(event: LoadEvent, requiredPermissions?: string[]) {
	try {
		// Check if initial setup is needed
		const needsSetup = await needsInitialSetup();
		if (needsSetup) {
			if (event.url.pathname !== '/setup') {
				throw redirect(303, '/setup');
			}
			return { user: null as any }; // Setup page doesn't need user
		}

		const user = await authenticateUser();

		// If specific permissions are required, check them
		if (requiredPermissions && requiredPermissions.length > 0) {
			const hasPermission = requiredPermissions.some((p) => user.permissions.includes(p));
			
			if (!hasPermission) {
				throw redirect(303, '/');
			}
		}

		// Check route-level permissions
		if (!canAccessRoute(event.url.pathname, user.permissions)) {
			throw redirect(303, '/');
		}

		return { user };
	} catch (error) {
		// If it's a redirect, re-throw it
		if (error instanceof Response) {
			throw error;
		}
		
		// Handle other errors
		const errorMsg = String(error);
		if (errorMsg.includes('USER_NOT_REGISTERED')) {
			throw redirect(303, '/locked');
		}
		
		throw redirect(303, '/locked');
	}
}
