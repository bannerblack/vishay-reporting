<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import { updateReport, deleteReport, type ReportData } from '$lib/db/adapters/report';
	import { assignTestToReport, unassignTestFromReport, updateTestOrder, createTest, type TestData } from '$lib/db/adapters/test';
	import { Save, FileSpreadsheet, GripVertical, Plus, X, ChevronUp, ChevronDown, Trash2 } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import TestForm from '$lib/components/test-form.svelte';
	import Combobox from '$lib/components/ui/combobox.svelte';

	let { data } = $props();
	let report = $state(data.report);
	let assignedTests = $state(data.tests);
	let fgs = $state(data.fgs);
	let availableTests = $state(data.availableTests);

	// Form data for metadata - separate editable state
	let attributes = $state(data.report.attributes);

	// Form for creating new tests
	let showTestForm = $state(false);

	async function handleCreateTest(testData: Omit<TestData, 'added_by'>) {
		try {
			const fullTestData: TestData = {
				...testData,
				report_id: report.id,
				fg_id: report.fg_id,
				order: assignedTests.length,
				added_by: null,
				source_type: testData.source_type ?? 'other',
				associated_test: testData.associated_test ?? null,
				manual_override: testData.manual_override ?? null
			};

			const created = await createTest(fullTestData);
			assignedTests = [...assignedTests, created];
			showTestForm = false;
		} catch (error) {
			console.error('Error creating test:', error);
			alert(`Error: ${error}`);
		}
	}

	// Available tests (not already assigned to this report)
	let unassignedTests = $derived(
		availableTests.filter(test => !assignedTests.some(assigned => assigned.id === test.id))
	);

	async function handleSaveMetadata() {
		try {
			const reportData: ReportData = {
				fg_id: report.fg_id,
				attributes,
				added_by: null
			};
			const updated = await updateReport(report.id, reportData);
			// Preserve the fg and tests properties from the complete report
			report = { ...updated, fg: report.fg, tests: report.tests };
			// Update form state with new values
			attributes = updated.attributes;
			alert('Report metadata saved!');
		} catch (error) {
			console.error('Error saving report:', error);
			alert(`Error: ${error}`);
		}
	}

	async function handleAssignTest(testId: number) {
		try {
			// Assign with order at the end of current list
			await assignTestToReport(testId, report.id);
			
			// Find the test in available tests
			const testToAdd = availableTests.find(t => t.id === testId);
			if (testToAdd) {
				// Add to assigned tests locally
				assignedTests = [...assignedTests, testToAdd];
			}
		} catch (error) {
			console.error('Error assigning test:', error);
			alert(`Error: ${error}`);
		}
	}

	async function handleUnassignTest(testId: number) {
		try {
			await unassignTestFromReport(testId);
			assignedTests = assignedTests.filter((t: any) => t.id !== testId);
		} catch (error) {
			console.error('Error unassigning test:', error);
			alert(`Error: ${error}`);
		}
	}

	function handleExportExcel() {
		// TODO: Call Tauri command to export Excel
		console.log('Export to Excel', { reportId: report.id, tests: assignedTests });
		alert('Excel export coming soon!');
	}

	async function handleMoveUp(index: number) {
		if (index > 0) {
			const temp = assignedTests[index];
			assignedTests[index] = assignedTests[index - 1];
			assignedTests[index - 1] = temp;
			
			// Update order in database
			try {
				await updateTestOrder(assignedTests[index].id, index);
				await updateTestOrder(assignedTests[index - 1].id, index - 1);
			} catch (error) {
				console.error('Error updating test order:', error);
				alert(`Error: ${error}`);
			}
		}
	}

	async function handleMoveDown(index: number) {
		if (index < assignedTests.length - 1) {
			const temp = assignedTests[index];
			assignedTests[index] = assignedTests[index + 1];
			assignedTests[index + 1] = temp;
			
			// Update order in database
			try {
				await updateTestOrder(assignedTests[index].id, index);
				await updateTestOrder(assignedTests[index + 1].id, index + 1);
			} catch (error) {
			console.error('Error updating test order:', error);
			alert(`Error: ${error}`);
		}
	}
}

async function handleDeleteReport() {
	if (confirm(`Are you sure you want to delete Report #${report.id}? This will also delete all associated tests.`)) {
		try {
			await deleteReport(report.id);
			goto('/manage/report');
		} catch (error) {
			console.error('Error deleting report:', error);
			alert(`Error: ${error}`);
		}
	}
}
</script><div class="container mx-auto p-6 space-y-6 col-span-3">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Edit Report #{report.id}</h1>
			<p class="text-muted-foreground">Manage report metadata and organize tests</p>
		</div>
	<div class="flex gap-2">
	<Button variant="outline" onclick={() => goto('/manage/report')}>
		Back to Reports
		</Button>
		<Button variant="destructive" onclick={handleDeleteReport}>
			<Trash2 class="mr-2 h-4 w-4" />
			Delete Report
		</Button>
		<Button onclick={handleExportExcel}>
			<FileSpreadsheet class="mr-2 h-4 w-4" />
			Export to Excel
		</Button>
		</div>
	</div>

	<!-- Report Metadata Card -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Report Metadata</Card.Title>
			<Card.Description>Edit the report's basic information</Card.Description>
		</Card.Header>
		<Card.Content class="space-y-4">
			<div>
				<h3 class="text-sm font-medium mb-1">Finished Good</h3>
				<p class="text-base">
				{#if fgs.find(fg => fg.id === report.fg_id)}
					{@const currentFg = fgs.find(fg => fg.id === report.fg_id)}
					{#if currentFg}
						{currentFg.fg} Rev {currentFg.rev} - {currentFg.customer}
					{/if}
				{:else}
					Unknown FG
				{/if}
				</p>
			</div>

			<div class="grid gap-2">
				<label for="attributes" class="text-sm font-medium">Attributes (JSON)</label>
				<Textarea 
					id="attributes" 
					bind:value={attributes} 
					placeholder="JSON attributes"
					rows={5}
				/>
			</div>
		</Card.Content>
		<Card.Footer>
			<Button onclick={handleSaveMetadata}>
				<Save class="mr-2 h-4 w-4" />
				Save Metadata
			</Button>
		</Card.Footer>
	</Card.Root>

	<!-- Available Tests from FG Card -->
	{#if unassignedTests.length > 0}
		<Card.Root>
			<Card.Header>
				<Card.Title>Available Tests from FG</Card.Title>
				<Card.Description>
					Select existing tests defined for this FG to add to the report
				</Card.Description>
			</Card.Header>
			<Card.Content>
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Test Type</Table.Head>
							<Table.Head>Frequency</Table.Head>
							<Table.Head>Voltage</Table.Head>
							<Table.Head>Range</Table.Head>
							<Table.Head>UoM</Table.Head>
							<Table.Head>Description</Table.Head>
							<Table.Head class="text-right">Actions</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each unassignedTests as test (test.id)}
							<Table.Row>
								<Table.Cell class="font-medium">{test.test_type}</Table.Cell>
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
								<Table.Cell class="max-w-xs truncate">{test.description ?? '-'}</Table.Cell>
								<Table.Cell class="text-right">
									<Button 
										size="sm" 
										onclick={() => handleAssignTest(test.id)}
									>
										<Plus class="mr-2 h-4 w-4" />
										Add to Report
									</Button>
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</Card.Content>
		</Card.Root>
	{/if}

	<!-- Assigned Tests Card -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title>Test Specifications (Report Body)</Card.Title>
					<Card.Description>
						Organize tests in the order they should appear in the report
					</Card.Description>
				</div>
				<Button onclick={() => showTestForm = !showTestForm}>
					<Plus class="mr-2 h-4 w-4" />
					{showTestForm ? 'Cancel' : 'Add Test'}
				</Button>
			</div>
		</Card.Header>
		<Card.Content class="space-y-4">
			<!-- New Test Form -->
			{#if showTestForm}
				<TestForm
					fgId={report.fg_id}
					fgNumber={report.fg.fg}
					reportId={report.id}
					onSubmit={handleCreateTest}
					onCancel={() => showTestForm = false}
				/>
			{/if}

			<!-- Tests Table -->
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-12">Order</Table.Head>
						<Table.Head>Test Type</Table.Head>
						<Table.Head>Frequency</Table.Head>
						<Table.Head>Voltage</Table.Head>
						<Table.Head>Range</Table.Head>
						<Table.Head>UoM</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if assignedTests.length === 0}
						<Table.Row>
							<Table.Cell colspan={7} class="text-center text-muted-foreground">
								No tests assigned to this report yet
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each assignedTests as test, index (test.id)}
							<Table.Row>
								<Table.Cell>
									<div class="flex items-center gap-2">
										<GripVertical class="h-4 w-4 text-muted-foreground cursor-move" />
										<span class="font-mono text-sm">{index + 1}</span>
									</div>
								</Table.Cell>
								<Table.Cell class="font-medium">{test.test_type}</Table.Cell>
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
									<div class="flex justify-end gap-1">
										<Button 
											variant="outline" 
											size="icon" 
											onclick={() => handleMoveUp(index)}
											disabled={index === 0}
											title="Move up"
										>
											<ChevronUp class="h-4 w-4" />
										</Button>
										<Button 
											variant="outline" 
											size="icon" 
											onclick={() => handleMoveDown(index)}
											disabled={index === assignedTests.length - 1}
											title="Move down"
										>
											<ChevronDown class="h-4 w-4" />
										</Button>
										<Button 
											variant="destructive" 
											size="icon" 
											onclick={() => handleUnassignTest(test.id)}
											title="Remove from report"
										>
											<X class="h-4 w-4" />
										</Button>
									</div>
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</Card.Content>
	</Card.Root>
</div>
