import type { PageLoad } from './$types';
import { authenticateUser } from '$lib/db/adapters/auth';
import { getAllEvents } from '$lib/db/database';

export const ssr = false;

export const load: PageLoad = async () => {
    try {
        const authenticatedUser = await authenticateUser();

        const events = await getAllEvents();

        return { 
            user: authenticatedUser,
            events
        };
    } catch (err) {
        console.error('Error loading page data:', err);
        return { 
            user: null,
            events: [],
        };
    }
};