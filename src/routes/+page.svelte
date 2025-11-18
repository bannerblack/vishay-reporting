<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	import * as Form from '$lib/components/ui/form/index';
	import Input from '$lib/components/ui/input/input.svelte';
    import Button from '$lib/components/ui/button/button.svelte';

	let confirmation_message: string = $state('');

	async function generateSpreadsheet() {
		const result = await invoke('generate_spreadsheet');

		if (result === 0) {
			confirmation_message = 'Spreadsheet generated successfully.';
		} else {
			confirmation_message = 'Failed to generate spreadsheet.';
		}
	}

	import { superForm, setMessage, setError } from 'sveltekit-superforms';
	import { _userSchema as userSchema } from '$types';
	import { zod4Client as zod } from 'sveltekit-superforms/adapters';

	let { data } = $props();

	const superFormObj = superForm(data.form, {
		SPA: true,
		validators: zod(userSchema),
		onUpdate({ form }) {
			alert('Form updated');
		}
	});

    let [username, name] = data.user;

	const { form: formData, errors, message, constraints, enhance } = superFormObj;
</script>

<h1>Hello, {username}!</h1>

<Button href="/locked">Locked</Button>

{#if confirmation_message}
	<p>{confirmation_message}</p>
{/if}

<button onclick={generateSpreadsheet} class="rounded border border-gray-300 px-4 py-2"
	>Generate Spreadsheet</button
>

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

<form method="POST" use:enhance>
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

	<Form.Field form={superFormObj} name="email">
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
</form>
