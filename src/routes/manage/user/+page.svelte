<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { createUser, updateUser, deleteUser, type UserData, type UserResponse } from '$lib/db/adapters/user';
	import { Plus, Pencil, Trash2, User, Search, Shield } from '@lucide/svelte';

	let { data } = $props();
	let users = $state<UserResponse[]>(data.users);
	let searchQuery = $state('');
	
	const filteredUsers = $derived(
		users.filter(user => 
			user.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
			user.username.toLowerCase().includes(searchQuery.toLowerCase()) ||
			user.permissions.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);
	
	let dialogOpen = $state(false);
	let editMode = $state(false);
	let currentUser = $state<UserResponse | null>(null);
	
	let formData = $state({
		name: '',
		username: '',
		preferences: '{"theme": "light"}',
		permissions: '[]',
		added_by: null as number | null
	});

	function openCreateDialog() {
		editMode = false;
		currentUser = null;
		formData = { 
			name: '', 
			username: '', 
			preferences: '{"theme": "light"}',
			permissions: '[]',
			added_by: null 
		};
		dialogOpen = true;
	}

	function openEditDialog(user: UserResponse) {
		editMode = true;
		currentUser = user;
		formData = { 
			name: user.name, 
			username: user.username, 
			preferences: user.preferences,
			permissions: user.permissions,
			added_by: null
		};
		dialogOpen = true;
	}

	function parsePermissions(permissionsStr: string): string[] {
		try {
			const parsed = JSON.parse(permissionsStr);
			return Array.isArray(parsed) ? parsed : [];
		} catch {
			return [];
		}
	}

	async function handleSubmit() {
		try {
			// Validate JSON format
			try {
				JSON.parse(formData.preferences);
				JSON.parse(formData.permissions);
			} catch {
				alert('Invalid JSON format in preferences or permissions');
				return;
			}

			if (editMode && currentUser) {
				const updated = await updateUser(currentUser.id, formData);
				users = users.map(u => u.id === updated.id ? updated : u);
			} else {
				const created = await createUser(formData);
				users = [...users, created];
			}
			dialogOpen = false;
		} catch (error) {
			console.error('Error saving user:', error);
			alert(`Error: ${error}`);
		}
	}

	async function handleDelete(id: number) {
		if (confirm('Are you sure you want to delete this user?')) {
			try {
				await deleteUser(id);
				users = users.filter(u => u.id !== id);
			} catch (error) {
				console.error('Error deleting user:', error);
				alert(`Error: ${error}`);
			}
		}
	}

	function getPermissionBadgeVariant(permission: string): "default" | "secondary" | "destructive" | "outline" {
		if (permission.includes('admin')) return 'destructive';
		if (permission.includes('manager')) return 'default';
		return 'secondary';
	}
</script>

<div class="container mx-auto py-6 col-span-3">
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title>Users</Card.Title>
					<Card.Description>Manage users and their permissions</Card.Description>
				</div>
				<Button onclick={openCreateDialog}>
					<Plus class="mr-2 h-4 w-4" />
					New User
				</Button>
			</div>
			<div class="mt-4">
				<div class="relative">
					<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Search by name, username, or permissions..."
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
						<Table.Head>Name</Table.Head>
						<Table.Head>Username</Table.Head>
						<Table.Head>Permissions</Table.Head>
						<Table.Head>Created</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if filteredUsers.length === 0}
						<Table.Row>
							<Table.Cell colspan={5} class="text-center text-muted-foreground">
								{searchQuery ? 'No users match your search.' : 'No users found. Create your first one!'}
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each filteredUsers as user (user.id)}
							<Table.Row class="hover:bg-muted/50">
								<Table.Cell class="font-medium">
									<div class="flex items-center gap-2">
										<User class="h-4 w-4 text-muted-foreground" />
										{user.name}
									</div>
								</Table.Cell>
								<Table.Cell>
									<code class="text-xs bg-muted px-2 py-1 rounded">{user.username}</code>
								</Table.Cell>
								<Table.Cell>
									<div class="flex gap-1 flex-wrap">
										{#each parsePermissions(user.permissions) as permission}
											<Badge variant={getPermissionBadgeVariant(permission)}>
												<Shield class="h-3 w-3 mr-1" />
												{permission}
											</Badge>
										{:else}
											<span class="text-xs text-muted-foreground">No permissions</span>
										{/each}
									</div>
								</Table.Cell>
								<Table.Cell>
									<span class="text-sm text-muted-foreground">
										{new Date(user.created_at).toLocaleDateString()}
									</span>
								</Table.Cell>
								<Table.Cell class="text-right">
									<div class="flex justify-end gap-2">
										<Button 
											variant="outline" 
											size="icon" 
											onclick={() => openEditDialog(user)}
										>
											<Pencil class="h-4 w-4" />
										</Button>
										<Button 
											variant="destructive" 
											size="icon" 
											onclick={() => handleDelete(user.id)}
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
			<Dialog.Title>{editMode ? 'Edit' : 'Create'} User</Dialog.Title>
			<Dialog.Description>
				{editMode ? 'Update the user details below.' : 'Enter the details for the new user.'}
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid gap-2">
				<label for="name" class="text-sm font-medium">Name</label>
				<Input id="name" bind:value={formData.name} placeholder="John Doe" />
			</div>
			<div class="grid gap-2">
				<label for="username" class="text-sm font-medium">Username</label>
				<Input id="username" bind:value={formData.username} placeholder="johndoe" />
			</div>
			<div class="grid gap-2">
				<label for="preferences" class="text-sm font-medium">Preferences (JSON)</label>
				<Textarea 
					id="preferences" 
					bind:value={formData.preferences} 
					placeholder={JSON.stringify({ theme: 'light' })}
					rows={3}
				/>
			</div>
			<div class="grid gap-2">
				<label for="permissions" class="text-sm font-medium">
					Permissions (JSON Array)
					<span class="text-xs text-muted-foreground ml-2">e.g., ["admin", "reviewer"]</span>
				</label>
				<Textarea 
					id="permissions" 
					bind:value={formData.permissions} 
					placeholder={JSON.stringify(['user', 'reviewer'])}
					rows={3}
				/>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => dialogOpen = false}>Cancel</Button>
			<Button onclick={handleSubmit}>{editMode ? 'Update' : 'Create'}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
