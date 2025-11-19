import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';

export const load: PageLoad = async () => {
	try {
		const [parts, overallStats] = await Promise.all([
			voltech.getAllParts(50),
			voltech.getOverallStats()
		]);

		return {
			parts,
			overallStats
		};
	} catch (error) {
		console.error('Error loading voltech parts:', error);
		throw error;
	}
};
