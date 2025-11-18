import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { invoke } from "@tauri-apps/api/core";

export const load = (async () => {
    const permissions : string[] = await invoke("get_user_roles");

    if (!permissions.includes("locked")) {
        redirect(302, '/return');
    }

    return {  };
    
}) satisfies PageLoad;