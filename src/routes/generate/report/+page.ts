
import type { PageLoad } from './$types';
import { getAllReportsWithFG} from '$lib/db/adapters/joins';
import { getAllFGs } from '$lib/db/database';

export const load = (async () => {
    const [reports, fgs] = await Promise.all([
        getAllReportsWithFG(),
        getAllFGs()
    ]);
    return { reports, fgs };
}) satisfies PageLoad;