<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { FileText, Package, ArrowLeft, Plus, Save } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import TestForm from '$lib/components/test-form.svelte';
	import { createTest, type TestData } from '$lib/db/adapters/test';
	import { createReport, type ReportData } from '$lib/db/adapters/report';

	let { data } = $props();
	let fg = $state(data.fg);
	let reports = $state(data.reports);
	let tests = $state(data.tests);

	// Test form state
	let showTestForm = $state(false);

	async function handleCreateTest(testData: Omit<TestData, 'added_by'>) {
		try {
			// Find the max order for tests belonging to this FG
			const maxOrder = tests.length > 0 
				? Math.max(...tests.map(t => t.order)) 
				: -1;
			
			const fullTestData: TestData = {
				...testData,
				fg_id: fg.id,
				report_id: null,
				added_by: null,
				order: maxOrder + 1,
				source_type: testData.source_type ?? 'other',
				associated_test: testData.associated_test ?? null,
				manual_override: testData.manual_override ?? null
			};

			const created = await createTest(fullTestData);
			tests = [...tests, created];
			showTestForm = false;
		} catch (error) {
			console.error('Error creating test:', error);
			alert(`Error: ${error}`);
		}
	}

	// Report form state
	let showReportForm = $state(false);
	let reportAttributes = $state('');

	async function handleCreateReport() {
		try {
			const reportData: ReportData = {
				fg_id: fg.id,
				attributes: reportAttributes,
				added_by: null
			};

		const created = await createReport(reportData);
		const createdWithFG = {
			...created,
			fg
		};
		reports = [...reports, createdWithFG];
		reportAttributes = '';
		showReportForm = false;			// Navigate to the new report page
			goto(`/report/${created.id}`);
		} catch (error) {
			console.error('Error creating report:', error);
			alert(`Error: ${error}`);
		}
	}
</script>

<div class="container mx-auto p-6 space-y-6 col-span-3">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<div class="flex items-center gap-3 mb-2">
				<Package class="h-8 w-8 text-primary" />
				<h1 class="text-3xl font-bold">{fg.fg} Rev {fg.rev}</h1>
			</div>
			<p class="text-muted-foreground">Customer: {fg.customer}</p>
		</div>
		<Button variant="outline" onclick={() => goto('/manage/fg')}>
			<ArrowLeft class="mr-2 h-4 w-4" />
			Back to FGs
		</Button>
	</div>

	<!-- Test Specifications Card -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title>Test Specifications</Card.Title>
					<Card.Description>All tests defined for this finished good</Card.Description>
				</div>
				<Button onclick={() => showTestForm = !showTestForm}>
					<Plus class="mr-2 h-4 w-4" />
					{showTestForm ? 'Cancel' : 'Add Test'}
				</Button>
			</div>
		</Card.Header>
		<Card.Content class="space-y-4">
			{#if showTestForm}
				<TestForm
					fgId={fg.id}
					onSubmit={handleCreateTest}
					onCancel={() => showTestForm = false}
					buttonText="Create Test"
				/>
			{/if}

			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Test Type</Table.Head>
						<Table.Head>Frequency</Table.Head>
						<Table.Head>Voltage</Table.Head>
						<Table.Head>Range</Table.Head>
						<Table.Head>UoM</Table.Head>
						<Table.Head>Description</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if tests.length === 0}
						<Table.Row>
							<Table.Cell colspan={6} class="text-center text-muted-foreground">
								No test specifications defined for this FG
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each tests as test, index (test.id + '-' + index)}
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
								<Table.Cell>
									<span class="text-sm text-muted-foreground">
										{test.description || '-'}
									</span>
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</Card.Content>
	</Card.Root>

	<!-- Reports Card -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title>Reports</Card.Title>
					<Card.Description>All reports created for this finished good</Card.Description>
				</div>
				<Button onclick={() => showReportForm = !showReportForm}>
					<Plus class="mr-2 h-4 w-4" />
					{showReportForm ? 'Cancel' : 'Create Report'}
				</Button>
			</div>
		</Card.Header>
		<Card.Content class="space-y-4">
			{#if showReportForm}
				<div class="border rounded-lg p-4 space-y-4 bg-muted/50">
					<h3 class="font-semibold">Create New Report</h3>
					<div class="grid gap-2">
						<label for="attributes" class="text-sm font-medium">Attributes (JSON)</label>
						<Textarea 
							id="attributes" 
							bind:value={reportAttributes} 
							placeholder="Optional JSON attributes"
							rows={3}
						/>
					</div>
					<div class="flex gap-2">
						<Button onclick={handleCreateReport}>
							<Save class="mr-2 h-4 w-4" />
							Create Report
						</Button>
						<Button variant="outline" onclick={() => {showReportForm = false; reportAttributes = '';}}>
							Cancel
						</Button>
					</div>
				</div>
			{/if}

			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Report ID</Table.Head>
						<Table.Head>Attributes</Table.Head>
						<Table.Head>Created</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if reports.length === 0}
						<Table.Row>
							<Table.Cell colspan={3} class="text-center text-muted-foreground">
								No reports created for this FG yet
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each reports as report (report.id)}
							<Table.Row 
								class="cursor-pointer hover:bg-muted/50" 
								onclick={() => goto(`/manage/report/${report.id}`)}
							>
								<Table.Cell class="font-medium">
									<div class="flex items-center gap-2">
										<FileText class="h-4 w-4 text-muted-foreground" />
										<span>#{report.id}</span>
									</div>
								</Table.Cell>
								<Table.Cell>
									<span class="text-sm text-muted-foreground truncate max-w-xs block">
										{report.attributes.substring(0, 80)}{report.attributes.length > 80 ? '...' : ''}
									</span>
								</Table.Cell>
								<Table.Cell>
									<Badge variant="outline">Report</Badge>
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</Card.Content>
	</Card.Root>
</div>
