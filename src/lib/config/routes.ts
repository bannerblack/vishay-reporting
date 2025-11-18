export interface RoutePermission {
	path: string;
	permissions?: string[]; // If undefined, route is public
	requireAll?: boolean; // If true, user must have ALL permissions; if false, ANY permission
}

export const ROUTE_PERMISSIONS: RoutePermission[] = [
	// Public routes
	{ path: '/' },
	{ path: '/locked' },
	{ path: '/setup' },

	// User management - requires admin (most specific first)
	{
		path: '/manage/user',
		permissions: ['admin'],
		requireAll: true
	},

	// Manage routes - require PE, QA, or Admin (covers all children automatically)
	{
		path: '/manage/fg',
		permissions: ['admin', 'pe', 'qa'],
		requireAll: false
	},
	{
		path: '/manage/report',
		permissions: ['admin', 'pe', 'qa'],
		requireAll: false
	},
	{
		path: '/manage/test',
		permissions: ['admin', 'pe', 'qa'],
		requireAll: false
	},
	{
		path: '/manage',
		permissions: ['admin', 'pe', 'qa'],
		requireAll: false
	}
];

export function canAccessRoute(pathname: string, userPermissions: string[]): boolean {
	// Sort routes by specificity (most specific first)
	const sortedRoutes = [...ROUTE_PERMISSIONS].sort((a, b) => b.path.length - a.path.length);
	
	const route = sortedRoutes.find((r) =>
		pathname === r.path || pathname.startsWith(r.path + '/')
	);

	if (!route || !route.permissions) {
		return true; // Public route
	}

	if (route.requireAll) {
		return route.permissions.every((p) => userPermissions.includes(p));
	} else {
		return route.permissions.some((p) => userPermissions.includes(p));
	}
}
