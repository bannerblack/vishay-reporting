<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { createReport, updateReport, deleteReport, type ReportData } from '$lib/db/adapters/report';
	import type { ReportWithFGResponse } from '$lib/db/adapters/joins';
	import type { FGResponse } from '$lib/db/adapters/fg';
	import { Plus, Pencil, Trash2, FileText, Search } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import Combobox from '$lib/components/ui/combobox.svelte';

	let { data } = $props();
	let reports = $state<ReportWithFGResponse[]>(data.reports);
	let fgs = $state<FGResponse[]>(data.fgs);
	let searchQuery = $state('');
	
	const filteredReports = $derived(
		reports.filter(report => 
			report.id.toString().includes(searchQuery) ||
			report.fg.fg.toLowerCase().includes(searchQuery.toLowerCase()) ||
			report.fg.rev.toLowerCase().includes(searchQuery.toLowerCase()) ||
			report.fg.customer.toLowerCase().includes(searchQuery.toLowerCase()) ||
			report.attributes.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);
	
	let dialogOpen = $state(false);
	let editMode = $state(false);
	let currentReport = $state<ReportWithFGResponse | null>(null);
	
	let formData = $state({
		fg_id: 0,
		attributes: '',
		addeded_by: null as number | null
	});

	function openCreateDialog() {
		editMode = false;
		currentReport = null;
		formData = { fg_id: 0, attributes: '', addeded_by: null };
		dialogOpen = true;
	}

	function openEditDialog(report: ReportWithFGResponse) {
		editMode = true;
		currentReport = report;
		formData = { 
			fg_id: report.fg_id, 
			attributes: report.attributes,
			addeded_by: null
		};
		dialogOpen = true;
	}

	async function handleSubmit() {
		try {
			if (formData.fg_id === 0) {
				alert('Please select an FG');
				return;
			}

			const reportData: ReportData = formData;

			if (editMode && currentReport) {
				const updated = await updateReport(currentReport.id, reportData);
				const fg = fgs.find(f => f.id === updated.fg_id)!;
				const updatedWithFG: ReportWithFGResponse = {
					...updated,
					fg
				};
				reports = reports.map(r => r.id === updatedWithFG.id ? updatedWithFG : r);
			} else {
				const created = await createReport(reportData);
				const fg = fgs.find(f => f.id === created.fg_id)!;
				const createdWithFG: ReportWithFGResponse = {
					...created,
					fg
				};
				reports = [...reports, createdWithFG];
			}
			dialogOpen = false;
		} catch (error) {
			console.error('Error saving report:', error);
			alert(`Error: ${error}`);
		}
	}

	async function handleDelete(id: number) {
		if (confirm('Are you sure you want to delete this report? This will also delete all associated tests.')) {
			try {
				await deleteReport(id);
				reports = reports.filter(r => r.id !== id);
			} catch (error) {
				console.error('Error deleting report:', error);
				alert(`Error: ${error}`);
			}
		}
	}
</script>

<div class="container mx-auto py-6 col-span-3">
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title>Reports</Card.Title>
					<Card.Description>Manage test reports for finished goods</Card.Description>
				</div>
				<Button onclick={openCreateDialog}>
					<Plus class="mr-2 h-4 w-4" />
					New Report
				</Button>
			</div>
			<div class="mt-4">
				<div class="relative">
					<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Search by report ID, FG number, customer, or attributes..."
						bind:value={searchQuery}
						class="pl-8"
					/>
				</div>
			</div>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Report ID</Table.Head>
						<Table.Head>FG Number</Table.Head>
						<Table.Head>Revision</Table.Head>
						<Table.Head>Customer</Table.Head>
						<Table.Head>Attributes</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if filteredReports.length === 0}
						<Table.Row>
							<Table.Cell colspan={6} class="text-center text-muted-foreground">
								{searchQuery ? 'No reports match your search.' : 'No reports found. Create your first one!'}
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each filteredReports as report (report.id)}
							<Table.Row class="cursor-pointer hover:bg-muted/50" onclick={() => goto(`/manage/report/${report.id}`)}>
								<Table.Cell class="font-medium">
									<div class="flex items-center gap-2">
										<FileText class="h-4 w-4 text-muted-foreground" />
										#{report.id}
									</div>
								</Table.Cell>
								<Table.Cell>{report.fg.fg}</Table.Cell>
								<Table.Cell>
									<Badge variant="outline">{report.fg.rev}</Badge>
								</Table.Cell>
								<Table.Cell>{report.fg.customer}</Table.Cell>
								<Table.Cell>
									<span class="text-sm text-muted-foreground truncate max-w-xs block">
										{report.attributes.substring(0, 50)}{report.attributes.length > 50 ? '...' : ''}
									</span>
								</Table.Cell>
								<Table.Cell class="text-right">
									<div class="flex justify-end gap-2">
										<Button 
											variant="outline" 
											size="icon" 
											onclick={(e) => {
												e.stopPropagation();
												openEditDialog(report);
											}}
										>
											<Pencil class="h-4 w-4" />
										</Button>
										<Button 
											variant="destructive" 
											size="icon" 
											onclick={(e) => {
												e.stopPropagation();
												handleDelete(report.id);
											}}
										>
											<Trash2 class="h-4 w-4" />
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

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Content class="max-w-2xl">
		<Dialog.Header>
			<Dialog.Title>{editMode ? 'Edit' : 'Create'} Report</Dialog.Title>
			<Dialog.Description>
				{editMode ? 'Update the report details below.' : 'Enter the details for the new report.'}
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
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
				<label for="attributes" class="text-sm font-medium">Attributes (JSON)</label>
				<Textarea 
					id="attributes" 
					bind:value={formData.attributes} 
					placeholder="JSON attributes"
					rows={5}
				/>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => dialogOpen = false}>Cancel</Button>
			<Button onclick={handleSubmit}>{editMode ? 'Update' : 'Create'}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>