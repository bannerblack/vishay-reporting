import type { PageLoad } from './$types';
import { getAllFGs } from '$lib/db/adapters/fg';
import { requireAuth } from '$lib/config/route-guard';

export const load = (async (event) => {
    await requireAuth(event); // Route permissions already defined in routes.ts

    const fgs = await getAllFGs();
    return { fgs };
}) satisfies PageLoad;