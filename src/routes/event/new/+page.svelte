<script lang="ts">
    import type { PageProps } from './$types';
    import { superForm } from 'sveltekit-superforms';
    import * as Form from '$lib/components/ui/form';
    import { Button } from '$lib/components/ui/button';
    import Input from '$lib/components/ui/input/input.svelte';
    import { newEventSchema } from '$types';
    import { event } from '$lib/db/database';
    import { goto } from '$app/navigation';

    import { zod4Client as zod } from 'sveltekit-superforms/adapters';

    let { data }: PageProps = $props();

    const superFormObj = superForm(data.form, {
		SPA: true,
		validators: zod(newEventSchema),
		async onUpdate({ form }) {
			if (form.valid) {
				try {
					await event.createEvent({
						originator_id: form.data.originator_id,
						target_id: form.data.target_id,
						report_id: form.data.report_id,
						comment: form.data.comment
					});
					goto('/');
				} catch (error) {
					console.error('Failed to create event:', error);
				}
			}
		}
	});

	const { form: formData, errors, message, constraints, enhance } = superFormObj;
</script>

<form method="POST" use:enhance>
	<Form.Field form={superFormObj} name="originator_id">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Originator ID</Form.Label>
				<Input type="number" {...props} bind:value={$formData.originator_id} />
			{/snippet}
		</Form.Control>
		<Form.Description />
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field form={superFormObj} name="target_id">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Target ID</Form.Label>
				<Input type="number" {...props} bind:value={$formData.target_id} />
			{/snippet}
		</Form.Control>
		<Form.Description />
		<Form.FieldErrors />
	</Form.Field>

    <Form.Field form={superFormObj} name="report_id">
        <Form.Control>
            {#snippet children({ props })}
                <Form.Label>Report ID</Form.Label>
                <Input type="number" bind:value={$formData.report_id} />
            {/snippet}
        </Form.Control>
        <Form.Description />
        <Form.FieldErrors />
    </Form.Field>

    <Form.Field form={superFormObj} name="comment">
        <Form.Control>
            {#snippet children({ props })}
                <Form.Label>Comment</Form.Label>
                <Input {...props} bind:value={$formData.comment} />
            {/snippet}
        </Form.Control>
        <Form.Description />
        <Form.FieldErrors />
    </Form.Field>

	<Button type="submit" class="mt-4 rounded border border-gray-300 px-4 py-2">Submit</Button>
</form>