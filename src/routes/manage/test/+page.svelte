<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { createTest, updateTest, deleteTest, assignTestToReport, unassignTestFromReport, type TestData, type TestResponse } from '$lib/db/adapters/test';
	import type { FGResponse } from '$lib/db/adapters/fg';
	import type { ReportResponse } from '$lib/db/adapters/report';
	import { Plus, Pencil, Trash2, Beaker, Link, Unlink, Search } from '@lucide/svelte';
	import Combobox from '$lib/components/ui/combobox.svelte';

	let { data } = $props();
	let tests = $state<TestResponse[]>(data.tests);
	let fgs = $state<FGResponse[]>(data.fgs);
	let reports = $state<ReportResponse[]>(data.reports);
	let searchQuery = $state('');
	
	const filteredTests = $derived(
		tests.filter(test => {
			const fg = fgs.find(f => f.id === test.fg_id);
			return (
				test.id.toString().includes(searchQuery) ||
				test.test_type.toLowerCase().includes(searchQuery.toLowerCase()) ||
				test.uo_m.toLowerCase().includes(searchQuery.toLowerCase()) ||
				(fg?.fg.toLowerCase().includes(searchQuery.toLowerCase()) ?? false) ||
				(test.description?.toLowerCase().includes(searchQuery.toLowerCase()) ?? false)
			);
		})
	);
	
	let dialogOpen = $state(false);
	let editMode = $state(false);
	let currentTest = $state<TestResponse | null>(null);
	
	let formData = $state({
		report_id: null as number | null,
		fg_id: 0,
		test_type: '',
		frequency: null as number | null,
		voltage: null as number | null,
		minimum: null as number | null,
		maximum: null as number | null,
		uo_m: '',
		primary_pins: null as string | null,
		secondary_pins: null as string | null,
		shorted_pins: null as string | null,
		description: null as string | null,
		added_by: null as number | null,
		order: null as number | null,
		source_type: 'other',
		associated_test: null as string | null,
		manual_override: null as boolean | null
	});

	function openCreateDialog() {
		editMode = false;
		currentTest = null;
		formData = {
			report_id: null,
			fg_id: 0,
			test_type: '',
			frequency: null,
			voltage: null,
			minimum: null,
			maximum: null,
			uo_m: '',
			primary_pins: null,
			secondary_pins: null,
			shorted_pins: null,
			description: null,
			added_by: null,
			order: null,
			source_type: 'other',
			associated_test: null,
			manual_override: null
		};
		dialogOpen = true;
	}

	function openEditDialog(test: TestResponse) {
		editMode = true;
		currentTest = test;
		formData = { 
			report_id: test.report_id ?? null,
			fg_id: test.fg_id,
			test_type: test.test_type,
			frequency: test.frequency ?? null,
			voltage: test.voltage ?? null,
			minimum: test.minimum ?? null,
			maximum: test.maximum ?? null,
			uo_m: test.uo_m,
			primary_pins: test.primary_pins ?? null,
			secondary_pins: test.secondary_pins ?? null,
			shorted_pins: test.shorted_pins ?? null,
			description: test.description ?? null,
			added_by: null,
			order: test.order ?? null,
			source_type: test.source_type,
			associated_test: test.associated_test ?? null,
			manual_override: test.manual_override ?? null
		};
		dialogOpen = true;
	}

	async function handleSubmit() {
		try {
			if (formData.fg_id === 0) {
				alert('Please select an FG');
				return;
			}

			// Calculate next order number for new tests
			let orderValue = formData.order ?? 0;
			if (!editMode) {
				// For new tests, find the max order for this FG/report combination
				const relevantTests = tests.filter(t => 
					t.fg_id === formData.fg_id && 
					t.report_id === formData.report_id
				);
				const maxOrder = relevantTests.length > 0 
					? Math.max(...relevantTests.map(t => t.order)) 
					: -1;
				orderValue = maxOrder + 1;
			}

			const testData: TestData = {
				...formData,
				order: orderValue,
				source_type: formData.source_type ?? 'other',
				associated_test: formData.associated_test ?? null,
				manual_override: formData.manual_override ?? null
			};

			if (editMode && currentTest) {
				const updated = await updateTest(currentTest.id, testData);
				tests = tests.map(t => t.id === updated.id ? updated : t);
			} else {
				const created = await createTest(testData);
				tests = [...tests, created];
			}
			dialogOpen = false;
		} catch (error) {
			console.error('Error saving test:', error);
			alert(`Error: ${error}`);
		}
	}

	async function handleDelete(id: number) {
		if (confirm('Are you sure you want to delete this test?')) {
			try {
				await deleteTest(id);
				tests = tests.filter(t => t.id !== id);
			} catch (error) {
				console.error('Error deleting test:', error);
				alert(`Error: ${error}`);
			}
		}
	}

	async function handleAssign(testId: number) {
		const reportId = prompt('Enter Report ID to assign to:');
		if (reportId) {
			try {
				const updated = await assignTestToReport(testId, parseInt(reportId));
				tests = tests.map(t => t.id === updated.id ? updated : t);
			} catch (error) {
				console.error('Error assigning test:', error);
				alert(`Error: ${error}`);
			}
		}
	}

	async function handleUnassign(testId: number) {
		if (confirm('Are you sure you want to unassign this test from its report?')) {
			try {
				const updated = await unassignTestFromReport(testId);
				tests = tests.map(t => t.id === updated.id ? updated : t);
			} catch (error) {
				console.error('Error unassigning test:', error);
				alert(`Error: ${error}`);
			}
		}
	}

	const fgOptions = $derived(fgs.map(fg => ({
		value: fg.id,
		label: `${fg.fg} Rev ${fg.rev}`
	})));

	const reportOptions = $derived(reports.map(r => ({
		value: r.id,
		label: `Report #${r.id} (FG ID: ${r.fg_id})`
	})));
</script>

<div class="container mx-auto py-6 col-span-3">
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title>Tests</Card.Title>
					<Card.Description>Manage test specifications for finished goods and reports</Card.Description>
				</div>
				<Button onclick={openCreateDialog}>
					<Plus class="mr-2 h-4 w-4" />
					New Test
				</Button>
			</div>
			<div class="mt-4">
				<div class="relative">
					<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Search by test type, FG number, or description..."
						bind:value={searchQuery}
						class="pl-8"
					/>
				</div>
			</div>
		</Card.Header>
		<Card.Content>
			<div class="overflow-x-auto">
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>ID</Table.Head>
							<Table.Head>Test Type</Table.Head>
							<Table.Head>FG</Table.Head>
							<Table.Head>Report</Table.Head>
							<Table.Head>Frequency</Table.Head>
							<Table.Head>Voltage</Table.Head>
							<Table.Head>Range</Table.Head>
							<Table.Head>UoM</Table.Head>
							<Table.Head class="text-right">Actions</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#if filteredTests.length === 0}
							<Table.Row>
								<Table.Cell colspan={9} class="text-center text-muted-foreground">
									{searchQuery ? 'No tests match your search.' : 'No tests found. Create your first one!'}
								</Table.Cell>
							</Table.Row>
						{:else}
							{#each filteredTests as test (test.id)}
								<Table.Row>
									<Table.Cell class="font-medium">
										<div class="flex items-center gap-2">
											<Beaker class="h-4 w-4 text-muted-foreground" />
											#{test.id}
										</div>
									</Table.Cell>
									<Table.Cell>{test.test_type}</Table.Cell>
									<Table.Cell>
										<Badge variant="outline">
											{fgs.find(f => f.id === test.fg_id)?.fg || `FG #${test.fg_id}`}
										</Badge>
									</Table.Cell>
									<Table.Cell>
										{#if test.report_id}
											<Badge>Report #{test.report_id}</Badge>
										{:else}
											<Badge variant="secondary">Unassigned</Badge>
										{/if}
									</Table.Cell>
									<Table.Cell>{test.frequency ?? '-'}</Table.Cell>
									<Table.Cell>{test.voltage ?? '-'}</Table.Cell>
									<Table.Cell>
										{#if test.minimum !== null && test.maximum !== null}
											{test.minimum} - {test.maximum}
										{:else}
											-
										{/if}
									</Table.Cell>
									<Table.Cell>{test.uo_m}</Table.Cell>
									<Table.Cell class="text-right">
										<div class="flex justify-end gap-2">
											{#if test.report_id}
												<Button variant="outline" size="icon" onclick={() => handleUnassign(test.id)} title="Unassign from report">
													<Unlink class="h-4 w-4" />
												</Button>
											{:else}
												<Button variant="outline" size="icon" onclick={() => handleAssign(test.id)} title="Assign to report">
													<Link class="h-4 w-4" />
												</Button>
											{/if}
											<Button variant="outline" size="icon" onclick={() => openEditDialog(test)}>
												<Pencil class="h-4 w-4" />
											</Button>
											<Button variant="destructive" size="icon" onclick={() => handleDelete(test.id)}>
												<Trash2 class="h-4 w-4" />
											</Button>
										</div>
									</Table.Cell>
								</Table.Row>
							{/each}
						{/if}
					</Table.Body>
				</Table.Root>
			</div>
		</Card.Content>
	</Card.Root>
</div>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Content class="max-w-3xl max-h-[90vh] overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>{editMode ? 'Edit' : 'Create'} Test</Dialog.Title>
			<Dialog.Description>
				{editMode ? 'Update the test details below.' : 'Enter the details for the new test specification.'}
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-2 gap-4">
				<div class="grid gap-2">
					<label for="fg" class="text-sm font-medium">Finished Good</label>
					<Combobox
						items={fgs.map(fg => ({
							value: fg.id,
							label: `${fg.fg} Rev ${fg.rev} - ${fg.customer}`
						}))}
						bind:value={formData.fg_id}
						onSelect={(value) => formData.fg_id = value as number}
						placeholder="Select FG..."
						searchPlaceholder="Search FGs..."
						emptyMessage="No FG found."
					/>
			</div>
			<div class="grid gap-2">
				<label for="report" class="text-sm font-medium">Report (Optional)</label>
				<Combobox
					items={[
						{ value: 0, label: 'No Report' },
						...reports.map(r => ({
							value: r.id,
							label: `Report #${r.id} (FG ID: ${r.fg_id})`
						}))
					]}
					bind:value={formData.report_id}
					onSelect={(value) => formData.report_id = (value === 0 ? null : value as number)}
					placeholder="Select report..."
					searchPlaceholder="Search reports..."
					emptyMessage="No report found."
				/>
			</div>
		</div>			<div class="grid grid-cols-2 gap-4">
				<div class="grid gap-2">
					<label for="test_type" class="text-sm font-medium">Test Type *</label>
					<Input id="test_type" bind:value={formData.test_type} placeholder="Resistance, Voltage, etc." />
				</div>
				<div class="grid gap-2">
					<label for="uo_m" class="text-sm font-medium">Unit of Measure *</label>
					<Input id="uo_m" bind:value={formData.uo_m} placeholder="Ohms, Volts, etc." />
				</div>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div class="grid gap-2">
					<label for="frequency" class="text-sm font-medium">Frequency</label>
					<Input id="frequency" type="number" bind:value={formData.frequency} placeholder="1000" />
				</div>
				<div class="grid gap-2">
					<label for="voltage" class="text-sm font-medium">Voltage</label>
					<Input id="voltage" type="number" bind:value={formData.voltage} placeholder="120" />
				</div>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div class="grid gap-2">
					<label for="minimum" class="text-sm font-medium">Minimum</label>
					<Input id="minimum" type="number" bind:value={formData.minimum} placeholder="100" />
				</div>
				<div class="grid gap-2">
					<label for="maximum" class="text-sm font-medium">Maximum</label>
					<Input id="maximum" type="number" bind:value={formData.maximum} placeholder="200" />
				</div>
			</div>

			<div class="grid grid-cols-3 gap-4">
				<div class="grid gap-2">
					<label for="primary_pins" class="text-sm font-medium">Primary Pins</label>
					<Input id="primary_pins" bind:value={formData.primary_pins} placeholder="1,2,3" />
				</div>
				<div class="grid gap-2">
					<label for="secondary_pins" class="text-sm font-medium">Secondary Pins</label>
					<Input id="secondary_pins" bind:value={formData.secondary_pins} placeholder="4,5,6" />
				</div>
				<div class="grid gap-2">
					<label for="shorted_pins" class="text-sm font-medium">Shorted Pins</label>
					<Input id="shorted_pins" bind:value={formData.shorted_pins} placeholder="7,8" />
				</div>
			</div>

			<div class="grid gap-2">
				<label for="description" class="text-sm font-medium">Description</label>
				<Textarea id="description" bind:value={formData.description} placeholder="Test description and notes" rows={3} />
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => dialogOpen = false}>Cancel</Button>
			<Button onclick={handleSubmit}>{editMode ? 'Update' : 'Create'}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
