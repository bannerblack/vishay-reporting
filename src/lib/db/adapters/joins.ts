import { invoke } from '@tauri-apps/api/core';
import type { FGResponse } from './fg';
import type { ReportResponse } from './report';
import type { TestResponse } from './test';

// ============================================================================
// Join Response Types
// ============================================================================

export interface FGWithReportsResponse {
	id: number;
	fg: string;
	rev: string;
	customer: string;
	reports: ReportResponse[];
}

export interface FGWithTestsResponse {
	id: number;
	fg: string;
	rev: string;
	customer: string;
	tests: TestResponse[];
}

export interface FGCompleteResponse {
	id: number;
	fg: string;
	rev: string;
	customer: string;
	reports: ReportResponse[];
	tests: TestResponse[];
}

export interface ReportWithFGResponse {
	id: number;
	fg_id: number;
	attributes: string;
	fg: FGResponse;
}

export interface ReportWithTestsResponse {
	id: number;
	fg_id: number;
	attributes: string;
	tests: TestResponse[];
}

export interface ReportCompleteResponse {
	id: number;
	fg_id: number;
	attributes: string;
	fg: FGResponse;
	tests: TestResponse[];
}

export interface TestWithFGResponse {
	id: number;
	report_id?: number | null;
	test_type: string;
	frequency?: number | null;
	voltage?: number | null;
	minimum?: number | null;
	maximum?: number | null;
	uo_m: string;
	fg: FGResponse;
}

export interface TestWithReportResponse {
	id: number;
	fg_id: number;
	test_type: string;
	frequency?: number | null;
	voltage?: number | null;
	minimum?: number | null;
	maximum?: number | null;
	uo_m: string;
	report?: ReportResponse | null;
}

export interface TestCompleteResponse {
	id: number;
	test_type: string;
	frequency?: number | null;
	voltage?: number | null;
	minimum?: number | null;
	maximum?: number | null;
	uo_m: string;
	fg: FGResponse;
	report?: ReportResponse | null;
}

// ============================================================================
// FG Join Operations
// ============================================================================

export async function getFGWithReports(fgId: number): Promise<FGWithReportsResponse> {
	try {
		return await invoke<FGWithReportsResponse>('get_fg_with_reports', { fgId });
	} catch (error) {
		throw new Error(`Failed to get FG with reports: ${error}`);
	}
}

export async function getFGWithTests(fgId: number): Promise<FGWithTestsResponse> {
	try {
		return await invoke<FGWithTestsResponse>('get_fg_with_tests', { fgId });
	} catch (error) {
		throw new Error(`Failed to get FG with tests: ${error}`);
	}
}

export async function getFGComplete(fgId: number): Promise<FGCompleteResponse> {
	try {
		return await invoke<FGCompleteResponse>('get_fg_complete', { fgId });
	} catch (error) {
		throw new Error(`Failed to get complete FG: ${error}`);
	}
}

// ============================================================================
// Report Join Operations
// ============================================================================

export async function getReportWithFG(reportId: number): Promise<ReportWithFGResponse> {
	try {
		return await invoke<ReportWithFGResponse>('get_report_with_fg', { reportId });
	} catch (error) {
		throw new Error(`Failed to get report with FG: ${error}`);
	}
}

export async function getReportWithTests(reportId: number): Promise<ReportWithTestsResponse> {
	try {
		return await invoke<ReportWithTestsResponse>('get_report_with_tests', { reportId });
	} catch (error) {
		throw new Error(`Failed to get report with tests: ${error}`);
	}
}

export async function getReportComplete(reportId: number): Promise<ReportCompleteResponse> {
	try {
		return await invoke<ReportCompleteResponse>('get_report_complete', { reportId });
	} catch (error) {
		throw new Error(`Failed to get complete report: ${error}`);
	}
}

// ============================================================================
// Test Join Operations
// ============================================================================

export async function getTestWithFG(testId: number): Promise<TestWithFGResponse> {
	try {
		return await invoke<TestWithFGResponse>('get_test_with_fg', { testId });
	} catch (error) {
		throw new Error(`Failed to get test with FG: ${error}`);
	}
}

export async function getTestWithReport(testId: number): Promise<TestWithReportResponse> {
	try {
		return await invoke<TestWithReportResponse>('get_test_with_report', { testId });
	} catch (error) {
		throw new Error(`Failed to get test with report: ${error}`);
	}
}

export async function getTestComplete(testId: number): Promise<TestCompleteResponse> {
	try {
		return await invoke<TestCompleteResponse>('get_test_complete', { testId });
	} catch (error) {
		throw new Error(`Failed to get complete test: ${error}`);
	}
}

// ============================================================================
// List Operations with Joins
// ============================================================================

export async function getAllReportsWithFG(): Promise<ReportWithFGResponse[]> {
	try {
		return await invoke<ReportWithFGResponse[]>('get_all_reports_with_fg');
	} catch (error) {
		throw new Error(`Failed to get all reports with FG: ${error}`);
	}
}

export async function getAllTestsByFG(fgId: number): Promise<TestResponse[]> {
	try {
		return await invoke<TestResponse[]>('get_all_tests_by_fg', { fgId });
	} catch (error) {
		throw new Error(`Failed to get all tests by FG: ${error}`);
	}
}

export async function getAllTestsByReport(reportId: number): Promise<TestResponse[]> {
	try {
		return await invoke<TestResponse[]>('get_all_tests_by_report', { reportId });
	} catch (error) {
		throw new Error(`Failed to get all tests by report: ${error}`);
	}
}

export async function getAvailableTestsForReport(reportId: number): Promise<TestResponse[]> {
	try {
		return await invoke<TestResponse[]>('get_available_tests_for_report', { reportId });
	} catch (error) {
		throw new Error(`Failed to get available tests for report: ${error}`);
	}
}
