import { invoke } from '@tauri-apps/api/core';
import type { PageLoad } from './$types';

export const load = (async () => {
    const [tests] = await Promise.all([
        invoke('get_manual_test_names', { fg: "132520" })
    ]);
    return { tests };
}) satisfies PageLoad;