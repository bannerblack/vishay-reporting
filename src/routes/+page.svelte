<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import Button from '$lib/components/ui/button/button.svelte';
	import CheckCircleIcon from '@lucide/svelte/icons/check-circle';
	import ClockIcon from '@lucide/svelte/icons/clock';
	import FileTextIcon from '@lucide/svelte/icons/file-text';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	const firstName = data.user?.name.split(' ')[1] || 'User';
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<!-- Welcome Header -->
	<div class="space-y-2">
		<h1 class="text-3xl font-bold">Welcome, {firstName}!</h1>
		<p class="text-muted-foreground">
			@{data.user?.username || 'unknown'} • {data.user?.permissions?.join(', ') || 'No permissions'}
		</p>
	</div>

	<!-- Events Card -->
	<Card.Root>
		<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<FileTextIcon class="h-5 w-5" />
				Your Events
			</Card.Title>
			<Card.Description>
				Events assigned to you that require attention
			</Card.Description>
		</Card.Header>
		<Card.Content>
			{#if data.events.length === 0}
				<div class="text-center py-8 text-muted-foreground">
					<p>No events assigned to you</p>
				</div>
			{:else}
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Status</Table.Head>
							<Table.Head>Report</Table.Head>
							<Table.Head>Comment</Table.Head>
							<Table.Head>Created</Table.Head>
							<Table.Head>Completed</Table.Head>
							<Table.Head>Actions</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each data.events as event (event.id)}
							<Table.Row>
								<Table.Cell>
									{#if event.complete}
										<Badge variant="outline" class="bg-green-50 text-green-700 border-green-200">
											<CheckCircleIcon class="h-3 w-3 mr-1" />
											Complete
										</Badge>
									{:else}
										<Badge variant="outline" class="bg-yellow-50 text-yellow-700 border-yellow-200">
											<ClockIcon class="h-3 w-3 mr-1" />
											Pending
										</Badge>
									{/if}
								</Table.Cell>
								<Table.Cell>
									<a href="/manage/report/{event.report_id}" class="text-primary hover:underline">
										Report #{event.report_id}
									</a>
								</Table.Cell>
								<Table.Cell class="max-w-xs truncate">
									{event.comment}
								</Table.Cell>
								<Table.Cell class="text-sm text-muted-foreground">
									{formatDate(event.created_at)}
								</Table.Cell>
								<Table.Cell class="text-sm text-muted-foreground">
									{#if event.completed_date}
										{formatDate(event.completed_date)}
									{:else}
										-
									{/if}
								</Table.Cell>
								<Table.Cell>
									<Button 
										variant="outline" 
										size="sm"
										href="/manage/report/{event.report_id}"
									>
										View Report
									</Button>
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			{/if}
		</Card.Content>
	</Card.Root>
</div>



<!-- Always use this form structure -->

<!-- <form method="POST" use:enhance>
 <Form.Field {form} name="email">
  <Form.Control>
   {#snippet children({ props })}
    <Form.Label>Email</Form.Label>
    <Input {...props} bind:value={$formData.email} />
   {/snippet}
  </Form.Control>
  <Form.Description />
  <Form.FieldErrors />
 </Form.Field>
</form> -->

<!-- <form method="POST" use:enhance>
	<Form.Field form={superFormObj} name="name">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Name</Form.Label>
				<Input {...props} bind:value={$formData.name} />
			{/snippet}
		</Form.Control>
		<Form.Description />
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field form={superFormObj} name="username">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Email</Form.Label>
				<Input {...props} bind:value={$formData.email} />
			{/snippet}
		</Form.Control>
		<Form.Description />
		<Form.FieldErrors />
	</Form.Field>

	<Button type="submit" class="mt-4 rounded border border-gray-300 px-4 py-2">Submit</Button>
</form> -->
