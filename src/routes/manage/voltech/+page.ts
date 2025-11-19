import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';

export const load: PageLoad = async () => {
	try {
		const [settings, errors] = await Promise.all([
			voltech.getVoltechSettings(),
			voltech.getVoltechErrors({ acknowledged: false })
		]);

		return {
			settings,
			errors
		};
	} catch (error) {
		console.error('Error loading voltech data:', error);
		throw error;
	}
};
