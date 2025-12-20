<script lang="ts">
	import { validateReport } from '$lib/db/database';
	import type { PageProps } from './$types';
	import { invoke } from '@tauri-apps/api/core';

	import * as Table from '$lib/components/ui/table/index';
    import { Button } from '$lib/components/ui/button/index';

	let { data }: PageProps = $props();

	let message = $state();

	import { debugVoltechQuery } from '$lib/db/adapters/debug';
	import { getFGWithTests } from '$lib/db/adapters/joins';

	async function handleValidation() {
		let response = await validateReport(1, 'DX150738', '11664-11667');
		const result = await debugVoltechQuery(
			'132520', // fg_number (not ID!)
			'11664-11667', // your serial range
			'002 LS Reading' // your associated_test
		);
		message = result;
	}

	let testData = $state();

	// Get Test Results by SN and PN
	async function getResults() {
		let response = await invoke('get_serialized', {
			fg: '132520',
			rev: 'FTA',
			startSerial: 11664,
			endSerial: 11667
		});
		testData = response;
	}

	// Generate and Open Excel Report
	async function generateReport() {
		// First get the tests for this FG
		let fgData = await getFGWithTests(1); // Assuming FG ID 1 for now
		let tests = fgData.tests;
		
		let response = await invoke('create_ba_report', {
			fg: '132520',
			rev: 'FTA',
			startSerial: 11664,
			endSerial: 11667,
			tests: tests
		});
		message = response;
	}
</script>

<div class="report-table col-span-3">
	<Table.Root>
		<Table.Header>
			<Table.Row>
				<Table.Head>FG</Table.Head>
				<Table.Head>Report</Table.Head>
				<Table.Head>Actions</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
            {#each data.reports as report}
			<Table.Row>
				<Table.Cell>{report.fg.fg}</Table.Cell>
				<Table.Cell>[Add Report Name]</Table.Cell>
				<Table.Cell><Button href="/generate/report/{report.id}">Generate Report</Button></Table.Cell>
			</Table.Row>
            {/each}
		</Table.Body>
	</Table.Root>
</div>

<pre>{JSON.stringify(data.reports, null, 2)}</pre>

{message}

<div class="results">
	<h1>Report Preview</h1>

	<h2>Voltech Results</h2>
	{#each testData as item}
		<div>{item.part} - {item.serial_num} - {item.pass_fail}</div>
		<!-- {JSON.stringify(item)} -->
	{/each}
</div>

<button onclick={handleValidation}>Validate Report</button>
<pre>{JSON.stringify(message, null, 2)}</pre>

<button onclick={getResults}>Get</button>
<button onclick={generateReport}>Generate Report</button>

<!-- What needs to be created:

A route at src/routes/generate/report/+page.svelte that:

Selects a report to generate
Calls validateReport() to check data availability
Shows available sessions/batches or prompts for serial range
Calls collectReportData() to gather the data
Calls generateAndSaveExcelReport() to create and download the Excel file -->
