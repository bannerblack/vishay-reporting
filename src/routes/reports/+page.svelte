<script lang="ts">
    import type { PageProps } from './$types';
    import { invoke } from '@tauri-apps/api/core';

    let { data }: PageProps = $props();

    let confirmation_message: string = $state('');

    async function generateSpreadsheet() {
		const result = await invoke('generate_spreadsheet');

		if (result === 0) {
			confirmation_message = 'Spreadsheet generated successfully.';
		} else {
			confirmation_message = 'Failed to generate spreadsheet.';
		}
	}
</script>

<h1>Route for Generating Reports</h1>

<button onclick={generateSpreadsheet} class="rounded border border-gray-300 px-4 py-2"
	>Generate Spreadsheet</button
>

{#if confirmation_message}
	<p>{confirmation_message}</p>
{/if}