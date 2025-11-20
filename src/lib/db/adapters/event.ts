import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types
// ============================================================================

export interface EventData {
    originator_id?: number | null;
    target_id?: number | null;
    report_id: number;
    comment: string;
}

export interface UpdateEventData {
    comment?: string | null;
    complete?: boolean | null;
}

export interface EventResponse {
    id: number;
    originator_id: number | null;
    target_id: number | null;
    report_id: number;
    comment: string;
    created_at: string;
    complete: boolean;
    completed_date: string | null;
}

// ============================================================================
// Adapter Functions
// ============================================================================

export async function createEvent(eventData: EventData): Promise<EventResponse> {
    try {
        return await invoke<EventResponse>('create_event', { eventData });
    } catch (error) {
        throw new Error(`Failed to create event: ${error}`);
    }
}

export async function getEvent(id: number): Promise<EventResponse> {
    try {
        return await invoke<EventResponse>('get_event', { id });
    } catch (error) {
        throw new Error(`Failed to get event: ${error}`);
    }
}

export async function getAllEvents(): Promise<EventResponse[]> {
    try {
        return await invoke<EventResponse[]>('get_all_events');
    } catch (error) {
        throw new Error(`Failed to get all events: ${error}`);
    }
}

export async function updateEvent(id: number, eventData: UpdateEventData): Promise<EventResponse> {
    try {
        return await invoke<EventResponse>('update_event', { id, eventData });
    } catch (error) {
        throw new Error(`Failed to update event: ${error}`);
    }
}

export async function deleteEvent(id: number): Promise<string> {
    try {
        return await invoke<string>('delete_event', { id });
    } catch (error) {
        throw new Error(`Failed to delete event: ${error}`);
    }
}

export async function completeEvent(id: number): Promise<EventResponse> {
    try {
        return await invoke<EventResponse>('complete_event', { id });
    } catch (error) {
        throw new Error(`Failed to complete event: ${error}`);
    }
}

export async function getEventsByReport(reportId: number): Promise<EventResponse[]> {
    try {
        return await invoke<EventResponse[]>('get_events_by_report', { reportId });
    } catch (error) {
        throw new Error(`Failed to get events by report: ${error}`);
    }
}

export async function getEventsByUser(userId: number): Promise<EventResponse[]> {
    try {
        return await invoke<EventResponse[]>('get_events_by_user', { userId });
    } catch (error) {
        throw new Error(`Failed to get events by user: ${error}`);
    }
}