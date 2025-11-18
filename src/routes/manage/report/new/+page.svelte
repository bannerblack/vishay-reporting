<script lang="ts">
	import type { PageProps } from './$types';
	import Button from '$lib/components/ui/button/button.svelte';
	import ButtonGroup from '$lib/components/ui/button-group/button-group.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { FieldGroup } from '$lib/components/ui/field';
	import * as Select from '$lib/components/ui/select/index.js';

	let { data }: PageProps = $props();

	type Column = {
		order: number;
		test_id: number;
		name: string;
	};

	let report: Column[] = $state([]);

	function addColumn() {
		report = [...report, { order: 0, test_id: 0, name: '' }];
	}

	function setTest(index: number, test_id: number, test_name: string) {
		report[index].test_id = test_id;
		report[index].name = test_name;

		report = report;
		console.log('Set');
	}
</script>

<h1>New Report</h1>

<div class="info col-span-3 flex flex-row gap-4">
	<!-- {JSON.stringify(report)} -->

	<h1>{data.fg.fg}</h1>
	<h1>{data.fg.rev}</h1>
	<h1>{data.fg.customer}</h1>

	{#each data.fg.attributes as att}
		{att}
	{/each}

	{#each data.fg.tests as test}
		<h1>{test.type}</h1>
		<h1>{test.frequency}</h1>
		<h1>{test.voltage}</h1>
		<h1>{test.min}</h1>
		<h1>{test.max}</h1>
		<h1>{test.output_uom}</h1>
		<h1>{test.prim_pins}</h1>
		<h1>{test.sec_pins}</h1>
		<h1>{test.shorted_pins}</h1>
	{/each}
</div>

<Button onclick={addColumn}>Add Column</Button>

<div class="columns col-span-3 flex flex-row">
	{#each report as column, index}
		<div class="flex flex-col">
			<div>Column: {index}</div>
            <div>Name: {column.name} {column.test_id}</div>

			<!-- Select Test -->
			<Select.Root
				type="single"
				onValueChange={(value) => {
					// find the test object by its value
					const test = data.fg.tests.find((t) => t.type === value);
					if (test) {
						setTest(index, test.id, test.type);
					}
				}}
			>
				<Select.Trigger class="w-[180px]">
					{column.name}
				</Select.Trigger>

				<Select.Content>
					{#each data.fg.tests as test}
						<Select.Item value={test.type}>
							{test.type}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
	{/each}
</div>
