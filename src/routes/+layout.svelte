<script lang="ts">
	import AppSidebar from "$lib/components/app-sidebar.svelte";
	import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
	import { Separator } from "$lib/components/ui/separator/index.js";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import { createUserContext, setUserContext } from '$lib/context/user-context.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();

	// Create and set user context
	const userContext = createUserContext();
	setUserContext(userContext);

	// Initialize authentication on mount
	onMount(async () => {
		await userContext.refresh();
		console.log('Auth state after refresh:', {
			isLoading: userContext.isLoading,
			isInitialSetup: userContext.isInitialSetup,
			isAuthenticated: userContext.isAuthenticated,
			user: userContext.user
		});

		// Redirect to setup if needed and not already there
		if (userContext.isInitialSetup && !window.location.pathname.includes('/setup')) {
			goto('/setup');
		}
	});

	// Debug reactive values
	$effect(() => {
		console.log('Layout reactive check:', {
			isLoading: userContext.isLoading,
			isInitialSetup: userContext.isInitialSetup,
			isAuthenticated: userContext.isAuthenticated
		});
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{#if userContext.isLoading}
	<div class="flex h-screen items-center justify-center">
		<div class="text-center">
			<div class="mb-4 text-lg">Loading...</div>
		</div>
	</div>
{:else if userContext.isInitialSetup}
	<!-- Initial setup screen will be shown by routing -->
	{@render children()}
{:else if !userContext.isAuthenticated}
	<div class="flex h-screen items-center justify-center">
		<div class="rounded-lg border bg-card p-8 text-center shadow-lg">
			<h1 class="mb-4 text-2xl font-bold">Access Denied</h1>
			<p class="text-muted-foreground">
				Your Windows account is not registered in the system. Please contact an administrator.
			</p>
		</div>
	</div>
{:else}
	<Sidebar.Provider>
		<AppSidebar />
		<Sidebar.Inset>
			<header class="flex h-16 shrink-0 items-center gap-2">
				<div class="flex items-center gap-2 px-4">
					<Sidebar.Trigger class="-ml-1" />
					<Separator orientation="vertical" class="mr-2 data-[orientation=vertical]:h-4" />
					<Breadcrumb.Root>
						<Breadcrumb.List>
							<Breadcrumb.Item class="hidden md:block">
								<Breadcrumb.Link href="##">LOUD BARK DEEP BITE</Breadcrumb.Link>
							</Breadcrumb.Item>
							<Breadcrumb.Separator class="hidden md:block" />
							<Breadcrumb.Item>
								<Breadcrumb.Page>Data Fetching</Breadcrumb.Page>
							</Breadcrumb.Item>
						</Breadcrumb.List>
					</Breadcrumb.Root>
				</div>
			</header>
			<div class="flex flex-1 flex-col gap-4 p-4 pt-0">
				<div class="grid auto-rows-min gap-4 md:grid-cols-3">
					{@render children()}
				</div>
				<div class="bg-muted/50 min-h-[100vh] flex-1 rounded-xl md:min-h-min"></div>
			</div>
		</Sidebar.Inset>
	</Sidebar.Provider>
{/if}
