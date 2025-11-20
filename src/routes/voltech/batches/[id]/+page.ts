import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';
import { error } from '@sveltejs/kit';

export const prerender = false;

export const load: PageLoad = async ({ params, url }) => {
    const batchNumber = params.id;
    const dateParam = url.searchParams.get('date');
    
    try {
        // If date parameter is provided, filter tests by date
        let tests;
        if (dateParam) {
            tests = await voltech.searchTests({
                batch: batchNumber,
                date_from: dateParam,
                date_to: dateParam
            });
        } else {
            // Otherwise get all tests for this batch
            tests = await voltech.getBatchTests(batchNumber);
        }
        
        if (tests.length === 0) {
            throw error(404, `No tests found for batch ${batchNumber}${dateParam ? ` on ${dateParam}` : ''}`);
        }
        
        // Calculate batch details from the actual tests
        const passed = tests.filter(t => t.pass_fail === 'PASS' || t.pass_fail === 'Pass').length;
        const failed = tests.length - passed;
        const passRate = tests.length > 0 ? (passed / tests.length) * 100 : 0;
        
        const batchDetails = {
            batch: batchNumber,
            part: tests[0].part,
            date: dateParam || tests[0].date,
            operator: tests[0].operator,
            total_tests: tests.length,
            passed,
            failed,
            pass_rate: passRate
        };
        
        return {
            batch: batchDetails,
            tests,
            dateFilter: dateParam
        };
    } catch (err) {
        console.error('Failed to load batch:', err);
        throw error(500, `Failed to load batch ${batchNumber}`);
    }
};