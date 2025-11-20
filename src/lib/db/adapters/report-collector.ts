import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types
// ============================================================================

export interface SingleTestResult {
    serial_number: string | null;
    batch: string;
    date: string;
    result: string; // "PASS" or "FAIL"
    measurements: Record<string, unknown>; // JSON object with test-specific measurements
}

export interface TestResultData {
    test_id: number;
    test_name: string;
    source_type: string;
    associated_test: string | null;
    spec_min: number | null;
    spec_max: number | null;
    spec_unit: string | null;
    results: SingleTestResult[];
}

export interface ReportData {
    report_id: number;
    fg_number: string;
    fg_revision: string;
    fg_customer: string;
    is_serialized: boolean;
    batch: string | null;
    serial_range: string | null;
    test_results: TestResultData[];
}

// ============================================================================
// Collection Functions
// ============================================================================

export async function collectReport(
    reportId: number,
    batch?: string,
    serialRange?: string,
    selectedDates?: string[]
): Promise<ReportData> {
    try {
        return await invoke<ReportData>('collect_report', {
            reportId,
            batch: batch || null,
            serialRange: serialRange || null,
            selectedDates: selectedDates || null
        });
    } catch (error) {
        throw new Error(`Failed to collect report data: ${error}`);
    }
}
