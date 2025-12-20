import { invoke } from '@tauri-apps/api/core';

export interface SampleRecord {
    part: string;
    serial_num: string;
    pass_fail: string;
    measurements_preview: string;
}

export interface DebugQueryResult {
    total_records: number;
    matching_fg: number;
    matching_fg_and_serials: number;
    matching_pass: number;
    matching_measurement: number;
    sample_records: SampleRecord[];
}

export async function debugVoltechQuery(
    fgNumber: string,
    serialRange: string,
    associatedTest: string
): Promise<DebugQueryResult> {
    try {
        return await invoke<DebugQueryResult>('debug_voltech_query', {
            fgNumber,
            serialRange,
            associatedTest
        });
    } catch (error) {
        throw new Error(`Failed to debug voltech query: ${error}`);
    }
}
