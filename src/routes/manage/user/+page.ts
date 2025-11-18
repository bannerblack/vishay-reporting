import type { PageLoad } from './$types';
import { getAllUsers } from '$lib/db/adapters/user';

export const load = (async () => {
    const users = await getAllUsers();
    return { users };
}) satisfies PageLoad;
