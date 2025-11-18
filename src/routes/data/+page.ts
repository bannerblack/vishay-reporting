import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';

export const load = (async () => {
    const batches = await voltech.getBatchesForPart('part123');
    return { batches };
}) satisfies PageLoad;