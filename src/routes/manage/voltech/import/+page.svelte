<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Alert from '$lib/components/ui/alert';
	import * as Dialog from '$lib/components/ui/dialog';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { Progress } from '$lib/components/ui/progress';
	import { Separator } from '$lib/components/ui/separator';
	import { voltech } from '$lib/db/database';
	import { getUserContext } from '$lib/context/user-context.svelte';
	import AlertCircleIcon from '@lucide/svelte/icons/alert-circle';
	import DatabaseIcon from '@lucide/svelte/icons/database';
	import DownloadIcon from '@lucide/svelte/icons/download';
	import RefreshCwIcon from '@lucide/svelte/icons/refresh-cw';
	import SettingsIcon from '@lucide/svelte/icons/settings';
	import CheckCircleIcon from '@lucide/svelte/icons/check-circle';
	import XCircleIcon from '@lucide/svelte/icons/x-circle';
	import { onMount, onDestroy } from 'svelte';

    import { tr } from '$lib/i18n/strings';

	let { data } = $props();
	const userContext = getUserContext();
	const isAdmin = $derived(userContext.hasPermission('admin'));

	// Form state
	let serverPath = $state(data.settings?.server_path || '');
	let dbPath = $state('');
	let networkPath = $state('');
	let confirmText = $state('');
	
	// Debug
	$effect(() => {
		console.log('Import page data:', data);
		console.log('Server path:', serverPath);
		console.log('Button should be enabled:', !!serverPath && !isResetting);
	});

	// UI state
	let isImporting = $state(false);
	let isResetting = $state(false);
	let isUpdatingPath = $state(false);
	let showResetDialog = $state(false);
	let statusMessage = $state('');
	let errorMessage = $state('');
	
	// Progress state
	let filesProcessed = $state(0);
	let recordsInserted = $state(0);
	let importErrors = $state<string[]>([]);

	// Event listener cleanup
	let unlistenProgress: (() => void) | null = null;

	onMount(async () => {
		// Listen for progress events
		unlistenProgress = await voltech.onVoltechBatchProgress((progress) => {
			filesProcessed = progress.files_processed;
			recordsInserted = progress.records_inserted;
			importErrors = progress.errors || [];
		});
	});

	onDestroy(() => {
		if (unlistenProgress) {
			unlistenProgress();
		}
	});

	function loadCurrentSettings() {
		if (data.settings) {
			serverPath = data.settings.server_path;
		}
		statusMessage = 'Settings loaded';
		setTimeout(() => statusMessage = '', 3000);
	}

	async function handleResetDatabase() {
		if (confirmText !== 'RESET') {
			errorMessage = 'You must type RESET to confirm';
			return;
		}

		isResetting = true;
		errorMessage = '';
		statusMessage = 'Resetting database...';

		try {
			const result = await voltech.resetVoltechDatabase();
			statusMessage = result;
			filesProcessed = 0;
			recordsInserted = 0;
			importErrors = [];
			showResetDialog = false;
			confirmText = '';
		} catch (error) {
			errorMessage = `Failed to reset database: ${error}`;
		} finally {
			isResetting = false;
		}
	}

	async function handleFullImport() {
		if (!serverPath) {
			errorMessage = 'Server path is required';
			return;
		}

		isImporting = true;
		errorMessage = '';
		statusMessage = 'Starting full import...';
		filesProcessed = 0;
		recordsInserted = 0;
		importErrors = [];

		try {
			const result = await voltech.fullImportVoltechFiles(
				serverPath,
				dbPath || undefined
			);
			statusMessage = result;
		} catch (error) {
			errorMessage = `Import failed: ${error}`;
		} finally {
			isImporting = false;
		}
	}

	async function handleUpdateServerPath() {
		if (!networkPath) {
			errorMessage = 'Network path is required';
			return;
		}

		isUpdatingPath = true;
		errorMessage = '';
		statusMessage = 'Updating server path...';

		try {
			const result = await voltech.updateServerPathSetting(networkPath);
			statusMessage = result;
			serverPath = networkPath;
		} catch (error) {
			errorMessage = `Failed to update path: ${error}`;
		} finally {
			isUpdatingPath = false;
		}
	}
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<!-- Header -->
	<div class="space-y-2">
		<h1 class="text-3xl font-bold">Full Historical Import</h1>
		<p class="text-muted-foreground">
			Build a complete voltech database from all available test files
		</p>
        <p>{tr('voltech.import.title')}</p>
	</div>

	<!-- Admin Warning -->
	{#if !isAdmin}
		<Alert.Root variant="destructive">
			<AlertCircleIcon class="h-4 w-4" />
			<Alert.Title>Admin Access Required</Alert.Title>
			<Alert.Description>
				Only administrators can perform full database imports and resets.
			</Alert.Description>
		</Alert.Root>
	{:else}
		<!-- Important Notes -->
		<Alert.Root>
			<AlertCircleIcon class="h-4 w-4" />
			<Alert.Title>Important Notes</Alert.Title>
			<Alert.Description>
				<ul class="list-disc list-inside space-y-1 mt-2">
					<li>Full imports can take several minutes to hours depending on file count</li>
					<li>The watcher must be stopped before importing</li>
					<li>Database reset is permanent and cannot be undone</li>
					<li>Relative path tracking allows moving database between local/network locations</li>
				</ul>
			</Alert.Description>
		</Alert.Root>

		<!-- Status Messages -->
		{#if statusMessage}
			<Alert.Root>
				<CheckCircleIcon class="h-4 w-4" />
				<Alert.Title>Status</Alert.Title>
				<Alert.Description>{statusMessage}</Alert.Description>
			</Alert.Root>
		{/if}

		{#if errorMessage}
			<Alert.Root variant="destructive">
				<XCircleIcon class="h-4 w-4" />
				<Alert.Title>Error</Alert.Title>
				<Alert.Description>{errorMessage}</Alert.Description>
			</Alert.Root>
		{/if}

		<!-- Import Settings -->
		<Card.Root>
			<Card.Header>
				<Card.Title class="flex items-center gap-2">
					<SettingsIcon class="h-5 w-5" />
					Import Settings
				</Card.Title>
				<Card.Description>
					Configure paths for the import operation
				</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-4">
				<div class="space-y-2">
					<Label for="serverPath">Source Path (Server Directory)</Label>
					<Input
						id="serverPath"
						bind:value={serverPath}
						placeholder="\\wsdv07\share\Results or C:\local_copy"
						disabled={isImporting}
					/>
					<p class="text-sm text-muted-foreground">
						Directory containing .atr test result files
					</p>
				</div>

				<div class="space-y-2">
					<Label for="dbPath">Target Database Path (Optional)</Label>
					<Input
						id="dbPath"
						bind:value={dbPath}
						placeholder="Leave blank to use current database"
						disabled={isImporting}
					/>
					<p class="text-sm text-muted-foreground">
						Specify custom database location for local compilation
					</p>
				</div>

				<div class="flex gap-2">
					<Button variant="outline" onclick={loadCurrentSettings} disabled={isImporting}>
						<RefreshCwIcon class="h-4 w-4 mr-2" />
						Load Current Settings
					</Button>
				</div>
			</Card.Content>
		</Card.Root>

		<!-- Database Operations -->
		<Card.Root>
			<Card.Header>
				<Card.Title class="flex items-center gap-2">
					<DatabaseIcon class="h-5 w-5" />
					Database Operations
				</Card.Title>
				<Card.Description>
					Reset database before importing
				</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-4">
				<Alert.Root variant="destructive">
					<AlertCircleIcon class="h-4 w-4" />
					<Alert.Title>Warning</Alert.Title>
					<Alert.Description>
						Resetting the database will permanently delete all test results, processed file records,
						settings, and errors. This action cannot be undone.
					</Alert.Description>
				</Alert.Root>

				<Button
					variant="destructive"
					onclick={() => showResetDialog = true}
					disabled={isImporting || isResetting}
				>
					<XCircleIcon class="h-4 w-4 mr-2" />
					Reset Database
				</Button>
			</Card.Content>
		</Card.Root>

		<!-- Full Import -->
		<Card.Root>
			<Card.Header>
				<Card.Title class="flex items-center gap-2">
					<DownloadIcon class="h-5 w-5" />
					Full Import
				</Card.Title>
				<Card.Description>
					Import all test files from the source directory
				</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-4">
				{#if isImporting}
					<div class="space-y-4">
						<div class="space-y-2">
							<div class="flex justify-between text-sm">
								<span>Files Processed:</span>
								<span class="font-mono">{filesProcessed}</span>
							</div>
							<div class="flex justify-between text-sm">
								<span>Records Inserted:</span>
								<span class="font-mono">{recordsInserted.toLocaleString()}</span>
							</div>
						</div>

						{#if importErrors.length > 0}
							<Alert.Root variant="destructive">
								<AlertCircleIcon class="h-4 w-4" />
								<Alert.Title>Errors Encountered</Alert.Title>
								<Alert.Description>
									<div class="max-h-32 overflow-y-auto mt-2">
										{#each importErrors as error}
											<div class="text-xs font-mono">{error}</div>
										{/each}
									</div>
								</Alert.Description>
							</Alert.Root>
						{/if}

						<div class="flex items-center gap-2">
							<RefreshCwIcon class="h-4 w-4 animate-spin" />
							<span class="text-sm">Import in progress...</span>
						</div>
					</div>
				{:else}
					<Button
						onclick={handleFullImport}
						disabled={!serverPath || isResetting}
						class="w-full"
					>
						<DownloadIcon class="h-4 w-4 mr-2" />
						Start Full Import
					</Button>
				{/if}

				{#if filesProcessed > 0 && !isImporting}
					<div class="p-4 bg-muted rounded-lg space-y-2">
						<div class="text-sm font-medium">Last Import Results:</div>
						<div class="grid grid-cols-2 gap-2 text-sm">
							<div>Files Processed:</div>
							<div class="font-mono text-right">{filesProcessed}</div>
							<div>Records Inserted:</div>
							<div class="font-mono text-right">{recordsInserted.toLocaleString()}</div>
						</div>
					</div>
				{/if}
			</Card.Content>
		</Card.Root>

		<!-- Deploy to Network -->
		<Card.Root>
			<Card.Header>
				<Card.Title class="flex items-center gap-2">
					<DatabaseIcon class="h-5 w-5" />
					Deploy to Network
				</Card.Title>
				<Card.Description>
					Update server path after moving database to network
				</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-4">
				<p class="text-sm text-muted-foreground">
					After building the database locally and copying it to the network location,
					update the server_path setting so the watcher uses the network directory.
				</p>

				<div class="space-y-2">
					<Label for="networkPath">Network Server Path</Label>
					<Input
						id="networkPath"
						bind:value={networkPath}
						placeholder="\\wsdv07\share\Results"
						disabled={isUpdatingPath}
					/>
				</div>

				<Button
					variant="outline"
					onclick={handleUpdateServerPath}
					disabled={!networkPath || isUpdatingPath}
				>
					{#if isUpdatingPath}
						<RefreshCwIcon class="h-4 w-4 mr-2 animate-spin" />
					{:else}
						<CheckCircleIcon class="h-4 w-4 mr-2" />
					{/if}
					Update Server Path
				</Button>
			</Card.Content>
		</Card.Root>
	{/if}
</div>

<!-- Reset Confirmation Dialog -->
<Dialog.Root bind:open={showResetDialog}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Reset Database</Dialog.Title>
			<Dialog.Description>
				This will permanently delete all data from the voltech database. Type RESET to confirm.
			</Dialog.Description>
		</Dialog.Header>
		<div class="space-y-4 py-4">
			<div class="space-y-2">
				<Label for="confirmText">Type RESET to confirm</Label>
				<Input
					id="confirmText"
					bind:value={confirmText}
					placeholder="RESET"
					disabled={isResetting}
				/>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => { showResetDialog = false; confirmText = ''; }}>
				Cancel
			</Button>
			<Button
				variant="destructive"
				onclick={handleResetDatabase}
				disabled={confirmText !== 'RESET' || isResetting}
			>
				{#if isResetting}
					<RefreshCwIcon class="h-4 w-4 mr-2 animate-spin" />
				{:else}
					<XCircleIcon class="h-4 w-4 mr-2" />
				{/if}
				Reset Database
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
