import type { PageLoad } from './$types';
import { voltech } from '$lib/db/database';
import { error } from '@sveltejs/kit';

export const prerender = false;

export const load: PageLoad = async ({ params }) => {
    const batchNumber = params.id;
    
    try {
        // Get ALL tests for this batch across all dates
        const tests = await voltech.getBatchTests(batchNumber);
        
        if (tests.length === 0) {
            throw error(404, `No tests found for batch ${batchNumber}`);
        }
        
        // Calculate aggregate stats from all tests
        const passed = tests.filter(t => t.pass_fail === 'PASS' || t.pass_fail === 'Pass').length;
        const failed = tests.length - passed;
        const passRate = tests.length > 0 ? (passed / tests.length) * 100 : 0;
        
        // Get unique dates and parts
        const uniqueDates = [...new Set(tests.map(t => t.date))];
        const uniqueParts = [...new Set(tests.map(t => t.part))];
        const uniqueOperators = [...new Set(tests.map(t => t.operator))];
        
        return {
            batchNumber,
            tests,
            stats: {
                total_tests: tests.length,
                passed,
                failed,
                pass_rate: passRate,
                dates: uniqueDates.sort(),
                parts: uniqueParts,
                operators: uniqueOperators
            }
        };
    } catch (err) {
        console.error('Failed to load batch:', err);
        throw error(500, `Failed to load batch ${batchNumber}`);
    }
};
