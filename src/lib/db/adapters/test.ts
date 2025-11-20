import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types
// ============================================================================

export interface TestData {
	report_id?: number | null;
	fg_id: number;
	test_type: string;
	frequency?: number | null;
	voltage?: number | null;
	minimum?: number | null;
	maximum?: number | null;
	uo_m: string;
	primary_pins?: string | null;
	secondary_pins?: string | null;
	shorted_pins?: string | null;
	description?: string | null;
	added_by?: number | null;
	order: number;
	source_type: string;
	associated_test?: string | null;
	manual_override?: boolean | null;
}

export interface TestResponse {
	id: number;
	report_id?: number | null;
	fg_id: number;
	test_type: string;
	frequency?: number | null;
	voltage?: number | null;
	minimum?: number | null;
	maximum?: number | null;
	uo_m: string;
	primary_pins?: string | null;
	secondary_pins?: string | null;
	shorted_pins?: string | null;
	description?: string | null;
	created_at: string;
	updated_at: string;
	order: number;
	source_type: string;
	associated_test?: string | null;
	manual_override?: boolean | null;
}

// ============================================================================
// Adapter Functions
// ============================================================================

export async function createTest(testData: TestData): Promise<TestResponse> {
	try {
		return await invoke<TestResponse>('create_test', { testData });
	} catch (error) {
		throw new Error(`Failed to create test: ${error}`);
	}
}

export async function getTest(id: number): Promise<TestResponse> {
	try {
		return await invoke<TestResponse>('get_test', { id });
	} catch (error) {
		throw new Error(`Failed to get test: ${error}`);
	}
}

export async function getAllTests(): Promise<TestResponse[]> {
	try {
		return await invoke<TestResponse[]>('get_all_tests');
	} catch (error) {
		throw new Error(`Failed to get all tests: ${error}`);
	}
}

export async function updateTest(id: number, testData: TestData): Promise<TestResponse> {
	try {
		return await invoke<TestResponse>('update_test', { id, testData });
	} catch (error) {
		throw new Error(`Failed to update test: ${error}`);
	}
}

export async function deleteTest(id: number): Promise<string> {
	try {
		return await invoke<string>('delete_test', { id });
	} catch (error) {
		throw new Error(`Failed to delete test: ${error}`);
	}
}

export async function assignTestToReport(testId: number, reportId: number): Promise<TestResponse> {
	try {
		return await invoke<TestResponse>('assign_test_to_report', { testId, reportId });
	} catch (error) {
		throw new Error(`Failed to assign test to report: ${error}`);
	}
}

export async function unassignTestFromReport(testId: number): Promise<TestResponse> {
	try {
		return await invoke<TestResponse>('unassign_test_from_report', { testId });
	} catch (error) {
		throw new Error(`Failed to unassign test from report: ${error}`);
	}
}

export async function updateTestOrder(testId: number, newOrder: number): Promise<string> {
	try {
		return await invoke<string>('update_test_order', { testId, newOrder });
	} catch (error) {
		throw new Error(`Failed to update test order: ${error}`);
	}
}
