import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types
// ============================================================================

export interface AvailableSession {
    date: string;
    batch: string;
    test_count: number;
}

export interface TestStatus {
    test_id: number;
    test_name: string;
    source_type: string;
    associated_test: string | null;
    has_data: boolean;
    record_count: number;
    available_sessions: AvailableSession[];
}

export interface ValidationResult {
    is_valid: boolean;
    is_serialized: boolean;
    tests: TestStatus[];
    message: string;
}

// ============================================================================
// Validation Functions
// ============================================================================

export async function validateReport(
    reportId: number,
    batch?: string,
    serialRange?: string
): Promise<ValidationResult> {
    try {
        return await invoke<ValidationResult>('validate_report', {
            reportId,
            batch: batch || null,
            serialRange: serialRange || null
        });
    } catch (error) {
        throw new Error(`Failed to validate report: ${error}`);
    }
}
