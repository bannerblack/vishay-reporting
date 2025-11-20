import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';

export const load = (async () => {
    try {
        // Get recent batches across all parts
        const recentBatches = await voltech.searchBatches({ limit: 50 });
        const overallStats = await voltech.getOverallStats();
        
        return {
            recentBatches,
            overallStats
        };
    } catch (error) {
        console.error('Failed to load batches:', error);
        return {
            recentBatches: [],
            overallStats: null
        };
    }
}) satisfies PageLoad;