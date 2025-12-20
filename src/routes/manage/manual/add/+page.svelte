<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';

	import { invoke } from '@tauri-apps/api/core';

	let loading = $state(false);

	let { data } = $props();
	// let reports = $state<ReportWithFGResponse[]>(data.reports);
	// let fgs = $state<FGResponse[]>(data.fgs);

	async function import_handler() {
		loading = true;
		let response = await invoke('import_manual_fg_folder', {fg: "132520"});
		loading = false;
	}
	
</script>

<Button onclick={import_handler}>Import</Button>

<pre>{JSON.stringify(data.tests, null, 2)}</pre>

{loading}