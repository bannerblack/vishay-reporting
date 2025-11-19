import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';

export const load: PageLoad = async () => {
	try {
		const [overallStats, dailyStats, operatorStats, failedTests] = await Promise.all([
			voltech.getOverallStats(),
			voltech.getDailyStats(undefined, undefined),
			voltech.getOperatorStats(undefined, undefined),
			voltech.getFailedTests(20)
		]);

		return {
			overallStats,
			dailyStats,
			operatorStats,
			failedTests
		};
	} catch (error) {
		console.error('Error loading voltech stats:', error);
		throw error;
	}
};
