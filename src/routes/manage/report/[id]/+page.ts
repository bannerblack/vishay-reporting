import { getReportComplete } from '$lib/db/adapters/joins';
import { getAllTestsByReport } from '$lib/db/adapters/joins';
import { getAllFGs } from '$lib/db/adapters/fg';
import { getAllTestsByFG } from '$lib/db/adapters/joins';
import type { PageLoad } from './$types';

export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	const reportId = parseInt(params.id);
	const report = await getReportComplete(reportId);
	const tests = await getAllTestsByReport(reportId);
	const fgs = await getAllFGs();
	
	// Load all tests for the current FG to allow adding existing tests
	let availableTests: any[] = [];
	if (report && report.fg_id) {
		availableTests = await getAllTestsByFG(report.fg_id);
	}
	
	return {
		report,
		tests,
		fgs,
		availableTests
	};
};
