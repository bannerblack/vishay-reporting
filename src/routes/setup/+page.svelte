<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import Input from '$lib/components/ui/input/input.svelte';
	import { createInitialAdmin, getSystemUser } from '$lib/db/adapters/auth';
	import { getUserContext } from '$lib/context/user-context.svelte';
	import { goto } from '$app/navigation';
	import ShieldIcon from '@lucide/svelte/icons/shield';

	const userContext = getUserContext();

	let adminPassword = $state('');
	let error = $state('');
	let systemUsername = $state('');
	let systemName = $state('');
	let loading = $state(false);

	// Get system user info
	$effect(() => {
		getSystemUser().then(([username, name]) => {
			systemUsername = username;
			systemName = name;
		});
	});

	async function handleSetup() {
		error = '';
		loading = true;
		
		try {
			await createInitialAdmin(adminPassword, {
				name: systemName,
				username: systemUsername,
				preferences: JSON.stringify({ theme: 'light' }),
				permissions: JSON.stringify(['admin']),
				added_by: null
			});

			// Refresh user context and redirect
			await userContext.refresh();
			goto('/');
		} catch (err) {
			error = String(err);
		} finally {
			loading = false;
		}
	}
</script>

<div class="flex h-screen items-center justify-center bg-muted/40">
	<Card.Root class="w-full max-w-md">
		<Card.Header>
			<div class="flex items-center gap-2 mb-2">
				<ShieldIcon class="h-6 w-6 text-primary" />
				<Card.Title>Initial Setup</Card.Title>
			</div>
			<Card.Description>
				Welcome! Set up the first administrator account for this system.
			</Card.Description>
		</Card.Header>
		<Card.Content class="space-y-4">
			<div class="space-y-2">
				<label for="user-info" class="text-sm font-medium">Your Windows Account</label>
				<div id="user-info" class="rounded-md border bg-muted p-3">
					<div class="font-medium">{systemName}</div>
					<div class="text-sm text-muted-foreground">@{systemUsername}</div>
				</div>
				<p class="text-xs text-muted-foreground">
					This account will be registered as the first administrator.
				</p>
			</div>

			<div class="space-y-2">
				<label for="password" class="text-sm font-medium">Admin Setup Password</label>
				<Input
					id="password"
					type="password"
					bind:value={adminPassword}
					placeholder="Enter admin password"
					disabled={loading}
				/>
				<p class="text-xs text-muted-foreground">
					Contact your system administrator for the setup password.
				</p>
			</div>

			{#if error}
				<div class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
					{error}
				</div>
			{/if}
		</Card.Content>
		<Card.Footer>
			<Button class="w-full" onclick={handleSetup} disabled={!adminPassword || loading}>
				{loading ? 'Creating...' : 'Create Admin Account'}
			</Button>
		</Card.Footer>
	</Card.Root>
</div>
