<form method="POST" use:enhance>
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
</form>

<!-- With Page.ts -->
import { error } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod4 } from 'sveltekit-superforms/adapters';
import { invoke } from '@tauri-apps/api/core';
// import { z } from 'zod';

import type { winUser } from '../types/types';
import { _userSchema } from '../types/types';

export const load = async ({ fetch }) => {
	//   const id = parseInt(params.id);
	const user: winUser = await invoke('get_user');
	const permissions: string[] = await invoke('get_user_roles');

	const request = await fetch(`https://jsonplaceholder.typicode.com/users/1`);
	if (request.status >= 400) throw error(request.status);

	const userData = await request.json();
	const form = await superValidate(userData, zod4(_userSchema));

	return { user, form, permissions };
};

<!-- Page -->
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

<!-- Form -->
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


<!-- No Page.ts (no preloaded form data) -->
<script lang="ts">
	import { superForm, defaults } from 'sveltekit-superforms';
	import { zod4, zod4Client } from 'sveltekit-superforms/adapters';
    import { z } from 'zod';
	import * as Form from '$lib/components/ui/form/index';
	import Input from '$lib/components/ui/input/input.svelte';

	export const userSchema = z.object({
		id: z.number().int().positive(),
		name: z.string().min(2),
		email: z.string().email()
	});

	const superFormObj = superForm(defaults(zod4(userSchema)), {
		SPA: true,
		validators: zod4Client(userSchema),
		onUpdate({ form }) {
			if (form.valid) {
                alert("Form called!")
				// TODO: Call an external API with form.data, await the result and update form
			}
		}
	});

    const { form: formData, errors, message, constraints, enhance } = superFormObj;
</script>