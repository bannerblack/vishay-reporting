import type { PageLoad } from './$types';
import { getAllTests } from '$lib/db/adapters/test';
import { getAllFGs } from '$lib/db/adapters/fg';
import { getAllReports } from '$lib/db/adapters/report';

export const load = (async () => {
    const [tests, fgs, reports] = await Promise.all([
        getAllTests(),
        getAllFGs(),
        getAllReports()
    ]);
    return { tests, fgs, reports };
}) satisfies PageLoad;
