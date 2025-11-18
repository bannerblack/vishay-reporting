<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Form from '$lib/components/ui/form';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { createFG, updateFG, deleteFG, type FGData, type FGResponse } from '$lib/db/adapters/fg';
	import { Plus, Pencil, Trash2, Package, Search } from '@lucide/svelte';
	import { goto } from '$app/navigation';

	let { data } = $props();
	let fgs = $state<FGResponse[]>(data.fgs);
	let searchQuery = $state('');
	
	const filteredFGs = $derived(
		fgs.filter(fg => 
			fg.fg.toLowerCase().includes(searchQuery.toLowerCase()) ||
			fg.rev.toLowerCase().includes(searchQuery.toLowerCase()) ||
			fg.customer.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);
	
	let dialogOpen = $state(false);
	let editMode = $state(false);
	let currentFG = $state<FGResponse | null>(null);
	
	let formData = $state({
		fg: '',
		rev: '',
		customer: ''
	});

	function openCreateDialog() {
		editMode = false;
		currentFG = null;
		formData = { fg: '', rev: '', customer: '' };
		dialogOpen = true;
	}

	function openEditDialog(fg: FGResponse) {
		editMode = true;
		currentFG = fg;
		formData = { fg: fg.fg, rev: fg.rev, customer: fg.customer };
		dialogOpen = true;
	}

	async function handleSubmit() {
		try {
			if (editMode && currentFG) {
				const updated = await updateFG(currentFG.id, formData);
				fgs = fgs.map(f => f.id === updated.id ? updated : f);
			} else {
				const created = await createFG(formData);
				fgs = [...fgs, created];
			}
			dialogOpen = false;
		} catch (error) {
			console.error('Error saving FG:', error);
			alert(`Error: ${error}`);
		}
	}

	async function handleDelete(id: number) {
		if (confirm('Are you sure you want to delete this FG? This will also delete all associated reports and tests.')) {
			try {
				await deleteFG(id);
				fgs = fgs.filter(f => f.id !== id);
			} catch (error) {
				console.error('Error deleting FG:', error);
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
					<Card.Title>Finished Goods (FG)</Card.Title>
					<Card.Description>Manage finished good part numbers and revisions</Card.Description>
				</div>
				<Button onclick={openCreateDialog}>
					<Plus class="mr-2 h-4 w-4" />
					New FG
				</Button>
			</div>
			<div class="mt-4">
				<div class="relative">
					<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Search by FG number, revision, or customer..."
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
						<Table.Head>FG Number</Table.Head>
						<Table.Head>Revision</Table.Head>
						<Table.Head>Customer</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if filteredFGs.length === 0}
						<Table.Row>
							<Table.Cell colspan={4} class="text-center text-muted-foreground">
								{searchQuery ? 'No finished goods match your search.' : 'No finished goods found. Create your first one!'}
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each filteredFGs as fg (fg.id)}
							<Table.Row class="cursor-pointer hover:bg-muted/50" onclick={() => goto(`/manage/fg/${fg.id}`)}>
								<Table.Cell class="font-medium">{fg.fg}</Table.Cell>
								<Table.Cell>
									<Badge variant="outline">{fg.rev}</Badge>
								</Table.Cell>
								<Table.Cell>{fg.customer}</Table.Cell>
								<Table.Cell class="text-right">
									<div class="flex justify-end gap-2">
										<Button 
											variant="outline" 
											size="icon" 
											onclick={(e) => {
												e.stopPropagation();
												openEditDialog(fg);
											}}
										>
											<Pencil class="h-4 w-4" />
										</Button>
										<Button 
											variant="destructive" 
											size="icon" 
											onclick={(e) => {
												e.stopPropagation();
												handleDelete(fg.id);
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
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>{editMode ? 'Edit' : 'Create'} Finished Good</Dialog.Title>
			<Dialog.Description>
				{editMode ? 'Update the finished good details below.' : 'Enter the details for the new finished good.'}
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid gap-2">
				<label for="fg" class="text-sm font-medium">FG Number</label>
				<Input id="fg" bind:value={formData.fg} placeholder="FG12345" />
			</div>
			<div class="grid gap-2">
				<label for="rev" class="text-sm font-medium">Revision</label>
				<Input id="rev" bind:value={formData.rev} placeholder="A" />
			</div>
			<div class="grid gap-2">
				<label for="customer" class="text-sm font-medium">Customer</label>
				<Input id="customer" bind:value={formData.customer} placeholder="Customer Name" />
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => dialogOpen = false}>Cancel</Button>
			<Button onclick={handleSubmit}>{editMode ? 'Update' : 'Create'}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

