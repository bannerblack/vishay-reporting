import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

// ============================================================================
// Types
// ============================================================================

export interface WatcherStatus {
	role: 'master' | 'follower' | 'none';
	master_user: string | null;
	is_active: boolean;
	is_paused: boolean;
	can_force_master: boolean;
	retry_count: number;
}

export interface VoltechSettings {
	server_path: string;
	db_path: string;
}

export interface VoltechSettingsResponse {
	server_path: string;
	db_path: string;
	last_monthly_scan: string | null;
}

export interface ParseError {
	id: number;
	file_path: string;
	error_message: string;
	line_number: number | null;
	timestamp: string;
	acknowledged: boolean;
}

export interface ErrorFilter {
	acknowledged?: boolean;
	date_range?: [string, string];
	file_path?: string;
}

export interface BatchListItem {
	batch: string;
	part: string;
	date: string;
	operator: string;
	total_tests: number;
	passed: number;
	failed: number;
	pass_rate: number;
}

export interface BatchSummary {
	batch: string;
	part: string;
	date: string;
	operator: string;
	total_tests: number;
	passed: number;
	failed: number;
	pass_rate: number;
}

export interface BatchSearchFilter {
	part?: string;
	date_from?: string;
	date_to?: string;
	operator?: string;
	limit?: number;
	offset?: number;
}

export interface PartSummary {
	part: string;
	total_batches: number;
	total_tests: number;
	passed: number;
	failed: number;
	pass_rate: number;
	first_date: string;
	last_date: string;
}

export interface PartListItem {
	part: string;
	total_tests: number;
	passed: number;
	failed: number;
	pass_rate: number;
}

export interface TestResult {
	id: number;
	file_path: string;
	result_num: number;
	part: string;
	batch: string;
	date: string;
	operator: string;
	pass_fail: string;
	serial_num: string;
	test_name: string;
	measurements: string; // JSON string
	created_at: string;
}

export interface TestSearchFilter {
	part?: string;
	batch?: string;
	operator?: string;
	date_from?: string;
	date_to?: string;
	pass_fail?: string;
	serial_num?: string;
	limit?: number;
	offset?: number;
}

export interface DailyStats {
	date: string;
	total_tests: number;
	passed: number;
	failed: number;
	pass_rate: number;
	total_parts: number;
	total_batches: number;
}

export interface OperatorStats {
	operator: string;
	total_tests: number;
	passed: number;
	failed: number;
	pass_rate: number;
	parts_tested: number;
	batches_completed: number;
}

export interface OverallStats {
	total_tests: number;
	total_parts: number;
	total_batches: number;
	total_operators: number;
	passed: number;
	failed: number;
	pass_rate: number;
}

export interface MaintenanceProgress {
	files_checked: number;
	files_updated: number;
	records_added: number;
}

export interface WatcherProgress {
	total_files: number;
	processed_files: number;
	total_records: number;
	error_count: number;
	current_file: string;
}

export interface BatchProgress {
	files_processed: number;
	records_inserted: number;
	errors: string[];
}

// ============================================================================
// File Processing Commands
// ============================================================================

export async function startVoltechWatcher(): Promise<WatcherStatus> {
	try {
		return await invoke<WatcherStatus>('start_voltech_watcher');
	} catch (error) {
		throw new Error(`Failed to start voltech watcher: ${error}`);
	}
}

export async function stopVoltechWatcher(): Promise<string> {
	try {
		return await invoke<string>('stop_voltech_watcher');
	} catch (error) {
		throw new Error(`Failed to stop voltech watcher: ${error}`);
	}
}

export async function pauseVoltechWatcher(): Promise<string> {
	try {
		return await invoke<string>('pause_voltech_watcher');
	} catch (error) {
		throw new Error(`Failed to pause voltech watcher: ${error}`);
	}
}

export async function resumeVoltechWatcher(): Promise<string> {
	try {
		return await invoke<string>('resume_voltech_watcher');
	} catch (error) {
		throw new Error(`Failed to resume voltech watcher: ${error}`);
	}
}

export async function getVoltechWatcherStatus(): Promise<WatcherStatus> {
	try {
		return await invoke<WatcherStatus>('get_voltech_watcher_status');
	} catch (error) {
		throw new Error(`Failed to get voltech watcher status: ${error}`);
	}
}

export async function importVoltechFiles(
	username: string,
	startDate: string,
	endDate: string
): Promise<string> {
	try {
		return await invoke<string>('import_voltech_files', { username, startDate, endDate });
	} catch (error) {
		throw new Error(`Failed to import voltech files: ${error}`);
	}
}

export async function forceAcquireVoltechMaster(): Promise<WatcherStatus> {
	try {
		return await invoke<WatcherStatus>('force_acquire_voltech_master');
	} catch (error) {
		throw new Error(`Failed to force acquire voltech master: ${error}`);
	}
}

export async function runVoltechMaintenanceScan(
	username: string,
	days?: number
): Promise<MaintenanceProgress> {
	try {
		return await invoke<MaintenanceProgress>('run_voltech_maintenance_scan', {
			username,
			days
		});
	} catch (error) {
		throw new Error(`Failed to run voltech maintenance scan: ${error}`);
	}
}

// ============================================================================
// Settings Commands
// ============================================================================

export async function getVoltechSettings(): Promise<VoltechSettingsResponse> {
	try {
		return await invoke<VoltechSettingsResponse>('get_voltech_settings');
	} catch (error) {
		throw new Error(`Failed to get voltech settings: ${error}`);
	}
}

export async function setVoltechSetting(
	key: string,
	value: string
): Promise<string> {
	try {
		return await invoke<string>('set_voltech_setting', { key, value });
	} catch (error) {
		throw new Error(`Failed to set voltech setting: ${error}`);
	}
}

export async function getAllVoltechSettings(): Promise<Record<string, string>> {
	try {
		return await invoke<Record<string, string>>('get_all_voltech_settings');
	} catch (error) {
		throw new Error(`Failed to get all voltech settings: ${error}`);
	}
}

export async function deleteVoltechSetting(key: string): Promise<string> {
	try {
		return await invoke<string>('delete_voltech_setting', { key });
	} catch (error) {
		throw new Error(`Failed to delete voltech setting: ${error}`);
	}
}

// ============================================================================
// Error Management Commands
// ============================================================================

export async function getVoltechErrors(filter: ErrorFilter): Promise<ParseError[]> {
	try {
		return await invoke<ParseError[]>('get_voltech_errors', { filter });
	} catch (error) {
		throw new Error(`Failed to get voltech errors: ${error}`);
	}
}

export async function acknowledgeVoltechErrors(ids: number[]): Promise<string> {
	try {
		return await invoke<string>('acknowledge_voltech_errors', { ids });
	} catch (error) {
		throw new Error(`Failed to acknowledge voltech errors: ${error}`);
	}
}

export async function acknowledgeFileErrors(filePath: string): Promise<string> {
	try {
		return await invoke<string>('acknowledge_file_errors', { filePath });
	} catch (error) {
		throw new Error(`Failed to acknowledge file errors: ${error}`);
	}
}

export async function clearAcknowledgedVoltechErrors(): Promise<string> {
	try {
		return await invoke<string>('clear_acknowledged_voltech_errors');
	} catch (error) {
		throw new Error(`Failed to clear acknowledged voltech errors: ${error}`);
	}
}

// ============================================================================
// Lock Management Commands
// ============================================================================

export async function getVoltechLockStatus(): Promise<string | null> {
	try {
		return await invoke<string | null>('get_voltech_lock_status');
	} catch (error) {
		throw new Error(`Failed to get voltech lock status: ${error}`);
	}
}

export async function forceReleaseVoltechLock(): Promise<string> {
	try {
		return await invoke<string>('force_release_voltech_lock');
	} catch (error) {
		throw new Error(`Failed to force release voltech lock: ${error}`);
	}
}

// ============================================================================
// Batch Query Commands
// ============================================================================

export async function getRecentBatchesForPart(
	part: string,
	limit?: number
): Promise<BatchListItem[]> {
	try {
		return await invoke<BatchListItem[]>('get_recent_batches_for_part', { part, limit });
	} catch (error) {
		throw new Error(`Failed to get recent batches for part: ${error}`);
	}
}

export async function getBatchDetails(batch: string): Promise<BatchSummary | null> {
	try {
		return await invoke<BatchSummary | null>('get_batch_details', { batch });
	} catch (error) {
		throw new Error(`Failed to get batch details: ${error}`);
	}
}

export async function getBatchTests(batch: string): Promise<TestResult[]> {
	try {
		return await invoke<TestResult[]>('get_batch_tests', { batch });
	} catch (error) {
		throw new Error(`Failed to get batch tests: ${error}`);
	}
}

export async function getBatchesForPart(part: string): Promise<string[]> {
	try {
		return await invoke<string[]>('get_batches_for_part', { part });
	} catch (error) {
		throw new Error(`Failed to get batches for part: ${error}`);
	}
}

export async function searchBatches(filter: BatchSearchFilter): Promise<BatchListItem[]> {
	try {
		return await invoke<BatchListItem[]>('search_batches', { filter });
	} catch (error) {
		throw new Error(`Failed to search batches: ${error}`);
	}
}

// ============================================================================
// Part Query Commands
// ============================================================================

export async function getAllParts(limit?: number): Promise<PartListItem[]> {
	try {
		return await invoke<PartListItem[]>('get_all_parts', { limit });
	} catch (error) {
		throw new Error(`Failed to get all parts: ${error}`);
	}
}

export async function getPartSummary(part: string): Promise<PartSummary | null> {
	try {
		return await invoke<PartSummary | null>('get_part_summary', { part });
	} catch (error) {
		throw new Error(`Failed to get part summary: ${error}`);
	}
}

export async function searchParts(pattern: string, limit?: number): Promise<string[]> {
	try {
		return await invoke<string[]>('search_parts', { pattern, limit });
	} catch (error) {
		throw new Error(`Failed to search parts: ${error}`);
	}
}

export async function getAllPartNumbers(): Promise<string[]> {
	try {
		return await invoke<string[]>('get_all_part_numbers');
	} catch (error) {
		throw new Error(`Failed to get all part numbers: ${error}`);
	}
}

export async function getPartStatsByDate(
	part: string,
	dateFrom: string,
	dateTo: string
): Promise<PartSummary | null> {
	try {
		return await invoke<PartSummary | null>('get_part_stats_by_date', {
			part,
			dateFrom,
			dateTo
		});
	} catch (error) {
		throw new Error(`Failed to get part stats by date: ${error}`);
	}
}

// ============================================================================
// Test Query Commands
// ============================================================================

export async function searchTests(filter: TestSearchFilter): Promise<TestResult[]> {
	try {
		return await invoke<TestResult[]>('search_tests', { filter });
	} catch (error) {
		throw new Error(`Failed to search tests: ${error}`);
	}
}

export async function getTestsBySerial(serialNum: string): Promise<TestResult[]> {
	try {
		return await invoke<TestResult[]>('get_tests_by_serial', { serialNum });
	} catch (error) {
		throw new Error(`Failed to get tests by serial: ${error}`);
	}
}

export async function getFailedTests(limit?: number): Promise<TestResult[]> {
	try {
		return await invoke<TestResult[]>('get_failed_tests', { limit });
	} catch (error) {
		// Return empty array if database is empty or query fails
		console.warn('Failed to get failed tests:', error);
		return [];
	}
}

export async function getTestById(id: number): Promise<TestResult | null> {
	try {
		return await invoke<TestResult | null>('get_test_by_id', { id });
	} catch (error) {
		throw new Error(`Failed to get test by id: ${error}`);
	}
}

export async function getTestsByBatch(batch: string): Promise<TestResult[]> {
	try {
		return await invoke<TestResult[]>('get_tests_by_batch', { batch });
	} catch (error) {
		throw new Error(`Failed to get tests by batch: ${error}`);
	}
}

export async function countTests(filter: TestSearchFilter): Promise<number> {
	try {
		return await invoke<number>('count_tests', { filter });
	} catch (error) {
		throw new Error(`Failed to count tests: ${error}`);
	}
}

export async function getRecentTests(limit: number): Promise<TestResult[]> {
	try {
		return await invoke<TestResult[]>('get_recent_tests', { limit });
	} catch (error) {
		throw new Error(`Failed to get recent tests: ${error}`);
	}
}

// ============================================================================
// Stats Query Commands
// ============================================================================

export async function getDailyStats(dateFrom?: string, dateTo?: string): Promise<DailyStats[]> {
	try {
		return await invoke<DailyStats[]>('get_daily_stats', { dateFrom, dateTo });
	} catch (error) {
		throw new Error(`Failed to get daily stats: ${error}`);
	}
}

export async function getOperatorStats(
	dateFrom?: string,
	dateTo?: string
): Promise<OperatorStats[]> {
	try {
		return await invoke<OperatorStats[]>('get_operator_stats', { dateFrom, dateTo });
	} catch (error) {
		throw new Error(`Failed to get operator stats: ${error}`);
	}
}

export async function getOverallStats(): Promise<OverallStats | null> {
	try {
		const stats = await invoke<OverallStats | null>('get_overall_stats');
		// Handle case where database is empty
		if (!stats) {
			return {
				total_tests: 0,
				total_parts: 0,
				total_batches: 0,
				total_operators: 0,
				passed: 0,
				failed: 0,
				pass_rate: 0
			};
		}
		return stats;
	} catch (error) {
		// Return empty stats on error instead of throwing
		console.warn('Failed to get overall stats:', error);
		return {
			total_tests: 0,
			total_parts: 0,
			total_batches: 0,
			total_operators: 0,
			passed: 0,
			failed: 0,
			pass_rate: 0
		};
	}
}

export async function getPartStats(part: string): Promise<OverallStats | null> {
	try {
		return await invoke<OverallStats | null>('get_part_stats', { part });
	} catch (error) {
		throw new Error(`Failed to get part stats: ${error}`);
	}
}

export async function getDateRange(): Promise<[string, string] | null> {
	try {
		return await invoke<[string, string] | null>('get_date_range');
	} catch (error) {
		throw new Error(`Failed to get date range: ${error}`);
	}
}

// ============================================================================
// Full Import Commands
// ============================================================================

export async function resetVoltechDatabase(): Promise<string> {
	try {
		return await invoke<string>('reset_voltech_database');
	} catch (error) {
		throw new Error(`Failed to reset voltech database: ${error}`);
	}
}

export async function fullImportVoltechFiles(
	serverPath: string,
	dbPath?: string
): Promise<string> {
	try {
		return await invoke<string>('full_import_voltech_files', {
			serverPath,
			dbPath: dbPath || null
		});
	} catch (error) {
		throw new Error(`Failed to import voltech files: ${error}`);
	}
}

export async function updateServerPathSetting(newPath: string): Promise<string> {
	try {
		return await invoke<string>('update_server_path_setting', { newPath });
	} catch (error) {
		throw new Error(`Failed to update server path: ${error}`);
	}
}

// ============================================================================
// Event Listeners
// ============================================================================

export async function onVoltechWatcherPaused(
	callback: (error: string) => void
): Promise<UnlistenFn> {
	return await listen<string>('voltech-watcher-paused', (event) => {
		callback(event.payload);
	});
}

export async function onVoltechWatcherResumed(callback: () => void): Promise<UnlistenFn> {
	return await listen('voltech-watcher-resumed', () => {
		callback();
	});
}

export async function onVoltechBatchProgress(
	callback: (progress: BatchProgress) => void
): Promise<UnlistenFn> {
	return await listen<BatchProgress>('voltech-batch-progress', (event) => {
		callback(event.payload);
	});
}

export async function onVoltechMaintenanceStart(callback: (days: number) => void): Promise<UnlistenFn> {
	return await listen<number>('voltech-maintenance-start', (event) => {
		callback(event.payload);
	});
}

export async function onVoltechMaintenanceProgress(
	callback: (progress: MaintenanceProgress) => void
): Promise<UnlistenFn> {
	return await listen<MaintenanceProgress>('voltech-maintenance-progress', (event) => {
		callback(event.payload);
	});
}

export async function onVoltechMaintenanceComplete(
	callback: (progress: MaintenanceProgress) => void
): Promise<UnlistenFn> {
	return await listen<MaintenanceProgress>('voltech-maintenance-complete', (event) => {
		callback(event.payload);
	});
}
