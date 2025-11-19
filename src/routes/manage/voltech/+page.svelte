<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Alert from '$lib/components/ui/alert';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Progress } from '$lib/components/ui/progress';
	import { voltech, type WatcherStatus, type VoltechSettingsResponse, type ParseError, type MaintenanceProgress } from '$lib/db/database';
	import { getUserContext } from '$lib/context/user-context.svelte';
	import { 
		Play, 
		Square, 
		Pause, 
		RotateCw, 
		Settings, 
		AlertCircle, 
		CheckCircle2, 
		Database,
		FolderOpen,
		Calendar,
		Server,
		Crown,
		Users,
		AlertTriangle,
		Loader2,
		RefreshCw,
		Lock,
		Unlock
	} from '@lucide/svelte';
	import { onMount } from 'svelte';

	let { data } = $props();
	const userContext = getUserContext();
	
	let watcherStatus = $state<WatcherStatus | null>(null);
	let settings = $state<VoltechSettingsResponse>(data.settings);
	let errors = $state<ParseError[]>(data.errors);
	let statusLoading = $state(true);
	let settingsDialogOpen = $state(false);
	let maintenanceDialogOpen = $state(false);
	let maintenanceInProgress = $state(false);
	let maintenanceProgress = $state<MaintenanceProgress | null>(null);
	
	// Form states - use derived to track settings changes
	let editedSettings = $derived({
		server_path: settings.server_path,
		db_path: settings.db_path
	});
	
	let maintenanceDays = $state(30);

	const isAdmin = $derived(userContext.hasPermission('admin'));
	const isMaster = $derived(watcherStatus?.role === 'master');
	const isFollower = $derived(watcherStatus?.role === 'follower');
	const canControl = $derived(isAdmin && watcherStatus);

	// Auto-refresh status every 10 seconds
	let statusInterval: number | undefined;

	async function loadWatcherStatus() {
		try {
			statusLoading = true;
			watcherStatus = await voltech.getVoltechWatcherStatus();
		} catch (error) {
			console.error('Error loading watcher status:', error);
		} finally {
			statusLoading = false;
		}
	}

	async function startWatcher() {
		try {
			watcherStatus = await voltech.startVoltechWatcher();
		} catch (error) {
			alert(`Failed to start watcher: ${error}`);
		}
	}

	async function stopWatcher() {
		if (!confirm('Stop the voltech file watcher? This will release the master lock.')) return;
		
		try {
			await voltech.stopVoltechWatcher();
			await loadWatcherStatus();
		} catch (error) {
			alert(`Failed to stop watcher: ${error}`);
		}
	}

	async function pauseWatcher() {
		try {
			await voltech.pauseVoltechWatcher();
			await loadWatcherStatus();
		} catch (error) {
			alert(`Failed to pause watcher: ${error}`);
		}
	}

	async function resumeWatcher() {
		try {
			await voltech.resumeVoltechWatcher();
			await loadWatcherStatus();
		} catch (error) {
			alert(`Failed to resume watcher: ${error}`);
		}
	}

	async function forceAcquireMaster() {
		if (!confirm('Force acquire master lock? This will disconnect the current master.')) return;
		
		try {
			watcherStatus = await voltech.forceAcquireVoltechMaster();
		} catch (error) {
			alert(`Failed to force acquire master: ${error}`);
		}
	}

	async function forceReleaseLock() {
		if (!confirm('Force release the watcher lock? This may disrupt active processing.')) return;
		
		try {
			await voltech.forceReleaseVoltechLock();
			await loadWatcherStatus();
		} catch (error) {
			alert(`Failed to force release lock: ${error}`);
		}
	}

	function openSettingsDialog() {
		editedSettings = {
			server_path: settings.server_path,
			db_path: settings.db_path
		};
		settingsDialogOpen = true;
	}

	async function saveSettings() {
		try {
			// Save each setting individually
			await voltech.setVoltechSetting('server_path', editedSettings.server_path);
			await voltech.setVoltechSetting('db_path', editedSettings.db_path);
			settings = await voltech.getVoltechSettings();
			settingsDialogOpen = false;
		} catch (error) {
			alert(`Failed to save settings: ${error}`);
		}
	}

	async function runMaintenanceScan() {
		alert('Maintenance scan is automatically run during file import. Use "Import Files" to process historical data.');
		maintenanceDialogOpen = false;
	}

	async function acknowledgeError(errorId: number) {
		try {
			await voltech.acknowledgeVoltechErrors([errorId]);
			errors = errors.filter(e => e.id !== errorId);
		} catch (error) {
			alert(`Failed to acknowledge error: ${error}`);
		}
	}

	async function acknowledgeAllErrors() {
		if (!confirm('Acknowledge all errors?')) return;
		
		try {
			const errorIds = errors.map(e => e.id);
			await voltech.acknowledgeVoltechErrors(errorIds);
			errors = [];
		} catch (error) {
			alert(`Failed to acknowledge errors: ${error}`);
		}
	}

	onMount(() => {
		loadWatcherStatus();
		
		// Refresh status every 10 seconds
		statusInterval = window.setInterval(loadWatcherStatus, 10000);
		
		return () => {
			if (statusInterval) clearInterval(statusInterval);
		};
	});

	function getRoleColor(role: string): string {
		switch (role) {
			case 'master': return 'bg-green-500';
			case 'follower': return 'bg-blue-500';
			default: return 'bg-gray-500';
		}
	}

	function getRoleBadgeVariant(role: string): "default" | "secondary" | "destructive" | "outline" {
		switch (role) {
			case 'master': return 'default';
			case 'follower': return 'secondary';
			default: return 'outline';
		}
	}
</script>

<div class="container mx-auto py-6 col-span-3 space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Voltech Management</h1>
			<p class="text-muted-foreground">Monitor and configure the Voltech test data integration</p>
		</div>
		{#if isAdmin}
			<Button onclick={openSettingsDialog}>
				<Settings class="mr-2 h-4 w-4" />
				Configure Settings
			</Button>
		{/if}
	</div>

	<!-- Watcher Status Card -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title class="flex items-center gap-2">
						<Server class="h-5 w-5" />
						Watcher Status
					</Card.Title>
					<Card.Description>Real-time file monitoring and processing</Card.Description>
				</div>
				<Button variant="outline" size="icon" onclick={loadWatcherStatus} disabled={statusLoading}>
					<RefreshCw class="h-4 w-4" />
				</Button>
			</div>
		</Card.Header>
		<Card.Content class="space-y-4">
			{#if statusLoading}
				<div class="flex items-center justify-center py-8">
					<Loader2 class="h-8 w-8 animate-spin text-muted-foreground" />
				</div>
			{:else if watcherStatus}
				<div class="grid gap-4 md:grid-cols-2">
					<div class="space-y-2">
						<Label class="text-muted-foreground">Role</Label>
						<div class="flex items-center gap-2">
							<Badge variant={getRoleBadgeVariant(watcherStatus.role)} class="text-base px-3 py-1">
								{#if watcherStatus.role === 'master'}
									<Crown class="h-4 w-4 mr-1" />
								{:else if watcherStatus.role === 'follower'}
									<Users class="h-4 w-4 mr-1" />
								{:else}
									<Server class="h-4 w-4 mr-1" />
								{/if}
								{watcherStatus.role.toUpperCase()}
							</Badge>
							{#if watcherStatus.is_active}
								<Badge variant="default">
									<CheckCircle2 class="h-3 w-3 mr-1" />
									Active
								</Badge>
							{/if}
							{#if watcherStatus.is_paused}
								<Badge variant="secondary">
									<Pause class="h-3 w-3 mr-1" />
									Paused
								</Badge>
							{/if}
						</div>
					</div>

					{#if watcherStatus.master_user}
						<div class="space-y-2">
							<Label class="text-muted-foreground">Master User</Label>
							<div class="flex items-center gap-2">
								<Users class="h-4 w-4 text-muted-foreground" />
								<code class="text-sm">{watcherStatus.master_user}</code>
							</div>
						</div>
					{/if}

					{#if watcherStatus.retry_count > 0}
						<div class="space-y-2">
							<Label class="text-muted-foreground">Retry Count</Label>
							<div class="flex items-center gap-2">
								<AlertTriangle class="h-4 w-4 text-yellow-500" />
								<span>{watcherStatus.retry_count}</span>
							</div>
						</div>
					{/if}
				</div>

				<Separator />

				<!-- Control Buttons -->
				<div class="flex gap-2 flex-wrap">
					{#if !watcherStatus.is_active}
						<Button onclick={startWatcher}>
							<Play class="mr-2 h-4 w-4" />
							Start Watcher
						</Button>
					{/if}

					{#if isMaster && watcherStatus.is_active}
						<Button variant="destructive" onclick={stopWatcher}>
							<Square class="mr-2 h-4 w-4" />
							Stop Watcher
						</Button>

						{#if watcherStatus.is_paused}
							<Button variant="default" onclick={resumeWatcher}>
								<Play class="mr-2 h-4 w-4" />
								Resume
							</Button>
						{:else}
							<Button variant="secondary" onclick={pauseWatcher}>
								<Pause class="mr-2 h-4 w-4" />
								Pause
							</Button>
						{/if}
					{/if}

					{#if isAdmin && (isFollower || watcherStatus.role === 'none')}
						<Button variant="outline" onclick={forceAcquireMaster}>
							<Crown class="mr-2 h-4 w-4" />
							Force Acquire Master
						</Button>
					{/if}

					{#if isAdmin && watcherStatus.master_user}
						<Button variant="outline" onclick={forceReleaseLock}>
							<Unlock class="mr-2 h-4 w-4" />
							Force Release Lock
						</Button>
					{/if}
				</div>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- Settings Overview Card -->
	<Card.Root>
		<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<Database class="h-5 w-5" />
				Configuration
			</Card.Title>
		</Card.Header>
		<Card.Content class="space-y-4">
			<div class="grid gap-4 md:grid-cols-2">
				<div class="space-y-2">
					<Label class="text-muted-foreground flex items-center gap-2">
						<FolderOpen class="h-4 w-4" />
						Server Path
					</Label>
					<code class="block text-sm bg-muted p-3 rounded-md break-all">
						{settings.server_path || 'Not configured'}
					</code>
				</div>

				<div class="space-y-2">
					<Label class="text-muted-foreground flex items-center gap-2">
						<Database class="h-4 w-4" />
						Database Path
					</Label>
					<code class="block text-sm bg-muted p-3 rounded-md break-all">
						{settings.db_path || 'Not configured'}
					</code>
				</div>

				{#if settings.last_monthly_scan}
					<div class="space-y-2">
						<Label class="text-muted-foreground flex items-center gap-2">
							<Calendar class="h-4 w-4" />
							Last Monthly Scan
						</Label>
						<div class="text-sm">
							{new Date(settings.last_monthly_scan).toLocaleString()}
						</div>
					</div>
				{/if}
			</div>

			{#if isAdmin}
				<Separator />
				<div class="flex gap-2">
					<Button variant="outline" onclick={() => maintenanceDialogOpen = true}>
						<RotateCw class="mr-2 h-4 w-4" />
						Run Maintenance Scan
					</Button>
				</div>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- Parse Errors Card -->
	{#if errors.length > 0}
		<Card.Root>
			<Card.Header>
				<div class="flex items-center justify-between">
					<div>
						<Card.Title class="flex items-center gap-2">
							<AlertCircle class="h-5 w-5 text-destructive" />
							Parse Errors ({errors.length})
						</Card.Title>
						<Card.Description>Recent errors from file processing</Card.Description>
					</div>
					<Button variant="outline" onclick={acknowledgeAllErrors}>
						<CheckCircle2 class="mr-2 h-4 w-4" />
						Acknowledge All
					</Button>
				</div>
			</Card.Header>
			<Card.Content>
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>File</Table.Head>
							<Table.Head>Error</Table.Head>
							<Table.Head>Line</Table.Head>
							<Table.Head>Time</Table.Head>
							<Table.Head class="text-right">Action</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each errors as error (error.id)}
							<Table.Row>
								<Table.Cell>
									<code class="text-xs">{error.file_path.split('\\').pop()}</code>
								</Table.Cell>
								<Table.Cell class="max-w-md">
									<span class="text-sm text-destructive line-clamp-2">{error.error_message}</span>
								</Table.Cell>
								<Table.Cell>
									{#if error.line_number}
										<code class="text-xs">{error.line_number}</code>
									{:else}
										<span class="text-muted-foreground">-</span>
									{/if}
								</Table.Cell>
								<Table.Cell>
									<span class="text-xs text-muted-foreground">
										{new Date(error.timestamp).toLocaleString()}
									</span>
								</Table.Cell>
								<Table.Cell class="text-right">
									<Button 
										variant="outline" 
										size="sm" 
										onclick={() => acknowledgeError(error.id)}
									>
										<CheckCircle2 class="h-3 w-3 mr-1" />
										Acknowledge
									</Button>
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</Card.Content>
		</Card.Root>
	{/if}
</div>

<!-- Settings Dialog -->
<Dialog.Root bind:open={settingsDialogOpen}>
	<Dialog.Content class="max-w-2xl">
		<Dialog.Header>
			<Dialog.Title>Configure Voltech Settings</Dialog.Title>
			<Dialog.Description>
				Update the server and database paths. Paths must be UNC network paths (\\server\share\path).
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<Alert.Root>
				<AlertTriangle class="h-4 w-4" />
				<Alert.Title>Admin Only</Alert.Title>
				<Alert.Description>
					These settings control where the watcher looks for .atr files. Changes take effect after restarting the watcher.
				</Alert.Description>
			</Alert.Root>

			<div class="grid gap-2">
				<Label for="server_path">Server Path</Label>
				<Input 
					id="server_path" 
					bind:value={editedSettings.server_path} 
					placeholder="\\server\voltech\data"
				/>
				<span class="text-xs text-muted-foreground">
					UNC path to the directory containing .atr files
				</span>
			</div>

			<div class="grid gap-2">
				<Label for="db_path">Database Path</Label>
				<Input 
					id="db_path" 
					bind:value={editedSettings.db_path} 
					placeholder="\\server\databases\voltech.sqlite"
				/>
				<span class="text-xs text-muted-foreground">
					UNC path to the shared voltech database file
				</span>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => settingsDialogOpen = false}>Cancel</Button>
			<Button onclick={saveSettings}>Save Settings</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<!-- Maintenance Scan Dialog -->
<Dialog.Root bind:open={maintenanceDialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Run Maintenance Scan</Dialog.Title>
			<Dialog.Description>
				Scan and process files from the last N days to catch any missed updates.
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid gap-2">
				<Label for="days">Number of Days</Label>
				<Input 
					id="days" 
					type="number" 
					bind:value={maintenanceDays} 
					min="1" 
					max="365"
				/>
				<span class="text-xs text-muted-foreground">
					Files modified within this many days will be checked and re-processed if needed.
				</span>
			</div>

			{#if maintenanceInProgress && maintenanceProgress}
				<div class="space-y-2">
					<Label>Progress</Label>
					<div class="space-y-1">
						<div class="flex justify-between text-sm">
							<span>Files Checked: {maintenanceProgress.files_checked}</span>
							<span>Updated: {maintenanceProgress.files_updated}</span>
						</div>
						<div class="text-xs text-muted-foreground">
							Records Added: {maintenanceProgress.records_added}
						</div>
					</div>
				</div>
			{/if}
		</div>
		<Dialog.Footer>
			<Button 
				variant="outline" 
				onclick={() => maintenanceDialogOpen = false}
				disabled={maintenanceInProgress}
			>
				Cancel
			</Button>
			<Button onclick={runMaintenanceScan} disabled={maintenanceInProgress}>
				{#if maintenanceInProgress}
					<Loader2 class="mr-2 h-4 w-4 animate-spin" />
					Scanning...
				{:else}
					<RotateCw class="mr-2 h-4 w-4" />
					Start Scan
				{/if}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
