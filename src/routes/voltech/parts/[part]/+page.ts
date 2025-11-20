import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';
import { error } from '@sveltejs/kit';

export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	try {
		const [partSummary, batches, partStats] = await Promise.all([
			voltech.getPartSummary(params.part),
			voltech.getBatchesForPart(params.part),
			voltech.getPartStats(params.part)
		]);

		if (!partSummary) {
			throw error(404, 'Part not found');
		}

		// Get recent batch details
		const recentBatches = await voltech.getRecentBatchesForPart(params.part, 10);

		return {
			part: params.part,
			summary: partSummary,
			batches,
			recentBatches,
			stats: partStats
		};
	} catch (err) {
		console.error('Error loading part details:', err);
		throw error(404, 'Part not found');
	}
};
