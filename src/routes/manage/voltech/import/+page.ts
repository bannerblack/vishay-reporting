import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';

export const load: PageLoad = async () => {
	try {
		const settings = await voltech.getVoltechSettings();
		return {
			settings
		};
	} catch (error) {
		console.error('Failed to load voltech settings:', error);
		return {
			settings: null
		};
	}
};
