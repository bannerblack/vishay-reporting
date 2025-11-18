import { getFG } from '$lib/db/adapters/fg';
import { getAllReportsWithFG, getAllTestsByFG } from '$lib/db/adapters/joins';
import type { PageLoad } from './$types';

import { requireAuth } from '$lib/config/route-guard';

export const prerender = false;

export const load: PageLoad = async (event) => {
	await requireAuth(event); 
	const fgId = parseInt(event.params.id);
	const fg = await getFG(fgId);
	const reports = await getAllReportsWithFG();
	const testsForFG = await getAllTestsByFG(fgId);
	
	// Filter reports to only those belonging to this FG
	const fgReports = reports.filter(r => r.fg_id === fgId);
	
	return {
		fg,
		reports: fgReports,
		tests: testsForFG
	};
};
