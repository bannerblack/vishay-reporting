import { invoke } from '@tauri-apps/api/core';
import type { winUser } from '$types';
import { requireAuth } from '$lib/config/route-guard';

export const load = async (event) => {
    await requireAuth(event); 
    //   const id = parseInt(params.id);
    const user: winUser = await invoke('get_system_user');
    const permissions: string[] = await invoke('get_user_roles');

    return { user, permissions };
};
