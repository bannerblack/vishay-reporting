<script lang="ts">
	import { validateReport } from '$lib/db/database';
	import type { PageProps } from './$types';
	import { invoke } from '@tauri-apps/api/core';
	import { newReportSchema } from '$types';

	import * as Table from '$lib/components/ui/table/index';
	import { Button } from '$lib/components/ui/button/index';
	// import * as Form from '$lib/components/ui/form/index';
	import Input from '$lib/components/ui/input/input.svelte';

	import { superForm } from 'sveltekit-superforms';
	import * as Form from '$lib/components/ui/form';

	import { newEventSchema } from '$types';
	import { event } from '$lib/db/database';
	import { goto } from '$app/navigation';

	import { zod4Client as zod } from 'sveltekit-superforms/adapters';

	let { data }: PageProps = $props();

	const superFormObj = superForm(data.form, {
		SPA: true,
		validators: zod(newReportSchema),
		async onUpdate({ form }) {
			if (form.valid) {
				try {
					// await event.createEvent({
					// 	originator_id: form.data.originator_id,
					// 	target_id: form.data.target_id,
					// 	report_id: form.data.report_id,
					// 	comment: form.data.comment
					// });
					if (form.data.sn_range) {
						await generateReport(form.data.sn_range, form.data.jobNumber, form.data.split, form.data.dateCode);
					}
				} catch (error) {
					console.error('Failed to create event:', error);
				}
			}
		}
	});

	const { form: formData, errors, message, constraints, enhance } = superFormObj;

	let resMessage = $state();

	import { debugVoltechQuery } from '$lib/db/adapters/debug';

	async function handleValidation() {
		let response = await validateReport(1, 'DX150738', '11664-11667');
		const result = await debugVoltechQuery(
			'132520', // fg_number (not ID!)
			'11664-11667', // your serial range
			'002 LS Reading' // your associated_test
		);
		resMessage = result;
	}

	let testData = $state();

	// Get Test Results by SN and PN
	async function getResults() {
		let response = await invoke('get_serialized', {
			fg: 'MTPL-2013-0023L',
			rev: 'FTA',
			startSerial: 26,
			endSerial: 95
		});
		testData = response;
	}

	// Generate and Open Excel Report
	async function generateReport(snRange: string, jobNumber: string, split: string, dateCode: string) {
		console.log('Generating Report');

		const [start, end] = snRange.split('-');

		let response = await invoke('create_ba_report', {
			fg: 'MTPL-2013-0023L',
			rev: 'FTA',
			startSerial: Number(start),
			endSerial: Number(end),
			tests: data.report.tests,
			jobNumber: jobNumber,
			split: split,
			dateCode: dateCode
		});
		resMessage = response;
	}
</script>

<pre>{JSON.stringify(data, null, 2)}</pre>

<div class="tests col-span-3">
	<h1>Tests</h1>
	{#each data.report.tests as test}
		{test.test_type} - {test.description} / Pins: {test.primary_pins} / {test.source_type} : {test.associated_test}
	{/each}
</div>

<div class="generate col-span-3">
	<h1>Generate Report</h1>
	<form method="POST" use:enhance>
		<Form.Field form={superFormObj} name="sn_range">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Serial Number Range</Form.Label>
					<Input type="text" {...props} bind:value={$formData.sn_range} />
				{/snippet}
			</Form.Control>
			<Form.Description>Format: 1000-2000</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={superFormObj} name="jobNumber">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Job Number</Form.Label>
					<Input type="text" {...props} bind:value={$formData.jobNumber} />
				{/snippet}
			</Form.Control>
			<Form.Description>Format: 1000-2000</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={superFormObj} name="split">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Split</Form.Label>
					<Input type="text" {...props} bind:value={$formData.split} />
				{/snippet}
			</Form.Control>
			<Form.Description>Example: FARNPI123456</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={superFormObj} name="dateCode">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Date Code</Form.Label>
					<Input type="text" {...props} bind:value={$formData.dateCode} />
				{/snippet}
			</Form.Control>
			<Form.Description>Example: {new Date().toISOString().slice(2, 4) + new Date().toISOString().slice(5, 7)}</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Button onclick={getResults}>Preview Report</Button>
		<Button type="submit" class="mt-4 rounded border border-gray-300 px-4 py-2">Submit</Button>
	</form>
</div>

<!-- <pre>{JSON.stringify(testData, null, 2)}</pre> -->

<hr />

<ul>
	{#each testData as test}
		<li>{test.part} - {test.serial_num}</li>
	{/each}
</ul>
