import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types
// ============================================================================

export interface FGData {
	fg: string;
	serialized: boolean;
	rev: string;
	customer: string;
}

export interface FGResponse {
	id: number;
	serialized: boolean;
	fg: string;
	rev: string;
	customer: string;
}

// ============================================================================
// Adapter Functions
// ============================================================================

export async function createFG(fgData: FGData): Promise<FGResponse> {
	try {
		return await invoke<FGResponse>('create_fg', { fgData });
	} catch (error) {
		throw new Error(`Failed to create FG: ${error}`);
	}
}

export async function getFG(id: number): Promise<FGResponse> {
	try {
		return await invoke<FGResponse>('get_fg', { id });
	} catch (error) {
		throw new Error(`Failed to get FG: ${error}`);
	}
}

export async function getFGByNumber(fgNumber: string): Promise<FGResponse> {
	try {
		return await invoke<FGResponse>('get_fg_by_number', { fgNumber });
	} catch (error) {
		throw new Error(`Failed to get FG by number: ${error}`);
	}
}

export async function getAllFGs(): Promise<FGResponse[]> {
	try {
		return await invoke<FGResponse[]>('get_all_fgs');
	} catch (error) {
		throw new Error(`Failed to get all FGs: ${error}`);
	}
}

export async function updateFG(id: number, fgData: FGData): Promise<FGResponse> {
	try {
		return await invoke<FGResponse>('update_fg', { id, fgData });
	} catch (error) {
		throw new Error(`Failed to update FG: ${error}`);
	}
}

export async function deleteFG(id: number): Promise<string> {
	try {
		return await invoke<string>('delete_fg', { id });
	} catch (error) {
		throw new Error(`Failed to delete FG: ${error}`);
	}
}
