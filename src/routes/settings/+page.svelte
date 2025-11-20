<script lang="ts">
	import { getUserContext } from '$lib/context/user-context.svelte';
	import { getPreferencesContext, THEMES, type ThemeName } from '$lib/context/preferences-context.svelte';
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import PaletteIcon from '@lucide/svelte/icons/palette';
	import GlobeIcon from '@lucide/svelte/icons/globe';
	import BellIcon from '@lucide/svelte/icons/bell';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const userContext = getUserContext();
	const preferencesContext = getPreferencesContext();

	// Parse permissions from JSON string
	const userPermissions = $derived.by(() => {
		try {
			const permsString = userContext.user?.permissions;
			if (!permsString || typeof permsString !== 'string') return [];
			const parsed = JSON.parse(permsString);
			return Array.isArray(parsed) ? parsed : [];
		} catch {
			return [];
		}
	});

	async function handleThemeChange(value: string) {
		if (!userContext.user) return;
		await preferencesContext.setTheme(userContext.user.id, value);
	}

	async function handleLanguageChange(value: string) {
		if (!userContext.user) return;
		await preferencesContext.setLanguage(userContext.user.id, value);
	}

	async function handleNotificationsChange(checked: boolean) {
		if (!userContext.user) return;
		await preferencesContext.update(userContext.user.id, { notifications: checked });
	}

	const themeLabels: Record<string, string> = {
		light: 'Light',
		dark: 'Dark',
		blue: 'Blue',
		green: 'Green',
		purple: 'Purple',
		system: 'System'
	};
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<div>
		<h1 class="text-3xl font-bold">Settings</h1>
		<p class="text-muted-foreground">Manage your application preferences</p>
	</div>

	<!-- Appearance Settings -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center gap-2">
				<PaletteIcon class="h-5 w-5" />
				<Card.Title>Appearance</Card.Title>
			</div>
			<Card.Description>Customize how the application looks and feels</Card.Description>
		</Card.Header>
		<Card.Content class="space-y-6">
			<div class="space-y-2">
				<Label for="theme">Theme</Label>
				<Select.Root
					type="single"
					value={preferencesContext.preferences.theme}
					onValueChange={(value) => value && handleThemeChange(value as string)}
				>
					<Select.Trigger id="theme" class="w-full">
						{themeLabels[preferencesContext.preferences.theme]}
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="light">Light</Select.Item>
						<Select.Item value="dark">Dark</Select.Item>
						<Select.Item value="blue">Blue</Select.Item>
						<Select.Item value="green">Green</Select.Item>
						<Select.Item value="purple">Purple</Select.Item>
						<Select.Item value="system">System</Select.Item>
					</Select.Content>
				</Select.Root>
				<p class="text-sm text-muted-foreground">
					Choose your preferred color theme. System will match your operating system's theme.
				</p>
			</div>

			<!-- Theme Preview -->
			<div class="rounded-lg border p-4 space-y-2">
				<p class="text-sm font-medium">Theme Preview</p>
				<div class="flex gap-2">
					<div class="h-12 w-12 rounded bg-background border"></div>
					<div class="h-12 w-12 rounded bg-primary"></div>
					<div class="h-12 w-12 rounded bg-secondary"></div>
					<div class="h-12 w-12 rounded bg-accent"></div>
					<div class="h-12 w-12 rounded bg-muted"></div>
				</div>
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Language Settings -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center gap-2">
				<GlobeIcon class="h-5 w-5" />
				<Card.Title>Language</Card.Title>
			</div>
			<Card.Description>Select your preferred language</Card.Description>
		</Card.Header>
		<Card.Content class="space-y-4">
			<div class="space-y-2">
				<Label for="language">Display Language</Label>
				<Select.Root
					type="single"
					value={preferencesContext.preferences.language}
					onValueChange={(value) => value && handleLanguageChange(value as string)}
				>
					<Select.Trigger id="language" class="w-full">
						English
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="en">English</Select.Item>
						<Select.Item value="es">Español</Select.Item>
						<Select.Item value="de">Deutsch</Select.Item>
						<Select.Item value="fr">Français</Select.Item>
						<Select.Item value="ja">日本語</Select.Item>
					</Select.Content>
				</Select.Root>
				<p class="text-sm text-muted-foreground">
					More languages coming soon. Currently only English is fully supported.
				</p>
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Notifications Settings -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center gap-2">
				<BellIcon class="h-5 w-5" />
				<Card.Title>Notifications</Card.Title>
			</div>
			<Card.Description>Manage how you receive notifications</Card.Description>
		</Card.Header>
		<Card.Content class="space-y-4">
			<div class="flex items-center justify-between">
				<div class="space-y-0.5">
					<Label for="notifications">Enable Notifications</Label>
					<p class="text-sm text-muted-foreground">Receive notifications about events and updates</p>
				</div>
				<Switch
					id="notifications"
					checked={preferencesContext.preferences.notifications}
					onCheckedChange={handleNotificationsChange}
				/>
			</div>
		</Card.Content>
	</Card.Root>

	<!-- User Info -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Account Information</Card.Title>
			<Card.Description>Your account details</Card.Description>
		</Card.Header>
		<Card.Content class="space-y-2">
			<div>
				<Label class="text-sm text-muted-foreground">Name</Label>
				<p class="font-medium">{userContext.user?.name}</p>
			</div>
			<div>
				<Label class="text-sm text-muted-foreground">Username</Label>
				<p class="font-medium">{userContext.user?.username}</p>
			</div>
			<div>
				<Label class="text-sm text-muted-foreground">Permissions</Label>
				<div class="flex gap-2 mt-1">
					{#each userPermissions as permission}
						<span class="px-2 py-1 text-xs bg-primary text-primary-foreground rounded">
							{permission}
						</span>
					{/each}
				</div>
			</div>
		</Card.Content>
	</Card.Root>
</div>
