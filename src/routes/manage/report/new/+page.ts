import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';

export const load = (async () => {
    const fg = await invoke("fake_data");
    return { fg };
}) satisfies PageLoad;