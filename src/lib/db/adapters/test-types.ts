import { invoke } from '@tauri-apps/api/core';

export interface AssociatedTests {
	voltech_tests: string[];
	manual_tests: string[];
}

/**
 * Get all available test type options
 * @returns Array of test type names (e.g., ["Inductance", "Leakage"])
 */
export async function getTestTypes(): Promise<string[]> {
	try {
		return await invoke<string[]>('get_test_types');
	} catch (error) {
		throw new Error(`Failed to get test types: ${error}`);
	}
}

/**
 * Find associated tests for a given FG and test type
 * @param fg - Finished Good number
 * @param testType - Test type name (e.g., "Inductance", "Leakage")
 * @returns Object containing arrays of test names from voltech and manual databases
 */
export async function findTestsForType(
	fg: string,
	testType: string
): Promise<AssociatedTests> {
	try {
		return await invoke<AssociatedTests>('find_tests_for_type', {
			fg,
			testType
		});
	} catch (error) {
		throw new Error(`Failed to find tests for type: ${error}`);
	}
}
