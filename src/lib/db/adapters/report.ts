import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types
// ============================================================================

export interface ReportData {
	fg_id: number;
	attributes: string;
	added_by?: number | null;
}

export interface ReportResponse {
	id: number;
	fg_id: number;
	attributes: string;
}

// ============================================================================
// Adapter Functions
// ============================================================================

export async function createReport(reportData: ReportData): Promise<ReportResponse> {
	try {
		return await invoke<ReportResponse>('create_report', { reportData });
	} catch (error) {
		throw new Error(`Failed to create report: ${error}`);
	}
}

export async function getReport(id: number): Promise<ReportResponse> {
	try {
		return await invoke<ReportResponse>('get_report', { id });
	} catch (error) {
		throw new Error(`Failed to get report: ${error}`);
	}
}

export async function getAllReports(): Promise<ReportResponse[]> {
	try {
		return await invoke<ReportResponse[]>('get_all_reports');
	} catch (error) {
		throw new Error(`Failed to get all reports: ${error}`);
	}
}

export async function updateReport(id: number, reportData: ReportData): Promise<ReportResponse> {
	try {
		return await invoke<ReportResponse>('update_report', { id, reportData });
	} catch (error) {
		throw new Error(`Failed to update report: ${error}`);
	}
}

export async function deleteReport(id: number): Promise<string> {
	try {
		return await invoke<string>('delete_report', { id });
	} catch (error) {
		throw new Error(`Failed to delete report: ${error}`);
	}
}
