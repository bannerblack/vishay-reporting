import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';

// ============================================================================
// Excel Generation Functions
// ============================================================================

/**
 * Generate Excel report and return as buffer
 */
export async function generateExcelReport(
    reportId: number,
    batch?: string,
    serialRange?: string,
    selectedDates?: string[]
): Promise<Uint8Array> {
    try {
        const buffer = await invoke<number[]>('generate_excel_report', {
            reportId,
            batch: batch || null,
            serialRange: serialRange || null,
            selectedDates: selectedDates || null
        });
        return new Uint8Array(buffer);
    } catch (error) {
        throw new Error(`Failed to generate Excel report: ${error}`);
    }
}

/**
 * Generate Excel report and prompt user to save
 * File operations are handled in Rust for better security and performance
 */
export async function generateAndSaveExcelReport(
    reportId: number,
    fgNumber: string,
    batch?: string,
    serialRange?: string,
    selectedDates?: string[]
): Promise<void> {
    try {
        // Create filename
        const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, -5);
        const identifier = batch || serialRange || 'report';
        const defaultFilename = `${fgNumber}_${identifier}_${timestamp}.xlsx`;
        
        // Prompt user to save
        const filePath = await save({
            defaultPath: defaultFilename,
            filters: [{
                name: 'Excel Files',
                extensions: ['xlsx']
            }]
        });
        
        if (filePath) {
            // File writing is now handled in Rust
            await invoke('save_excel_report', {
                reportId,
                filePath,
                batch: batch || null,
                serialRange: serialRange || null,
                selectedDates: selectedDates || null
            });
        }
    } catch (error) {
        throw new Error(`Failed to save Excel report: ${error}`);
    }
}
