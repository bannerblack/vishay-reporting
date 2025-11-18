<script lang="ts">
	import { superForm, defaults } from 'sveltekit-superforms';
	import { zod4, zod4Client } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { string, z } from 'zod';
	import FormElementField from '$lib/components/ui/form/form-element-field.svelte';
	import { invoke } from '@tauri-apps/api/core'

	import { newFgSchema } from '$types';

	async function submit_data(formData: any) {
		try {
			// Create FG
			const fgData = {
				fg: formData.fg,
				rev: formData.rev,
				customer: formData.customer
			};
			const createdFg: any = await invoke("create_fg", { fgData });
			console.log("FG created:", createdFg);

			// Create Report with attributes
			const reportData = {
				fg_id: createdFg.id,
				attributes: JSON.stringify(formData.attributes),
				added_by: null
			};
			const createdReport: any = await invoke("create_report", { reportData });
			console.log("Report created:", createdReport);

			// Create Tests
			for (const test of formData.tests) {
				const testData = {
					report_id: createdReport.id,
					fg_id: createdFg.id,
					test_type: test.type,
					frequency: test.frequency || null,
					voltage: test.voltage || null,
					minimum: test.min || null,
					maximum: test.max || null,
					uo_m: test.output_uom,
					primary_pins: test.prim_pins || null,
					secondary_pins: test.sec_pins || null,
					shorted_pins: test.shorted_pins || null,
					description: test.description || null,
					added_by: null
				};
				await invoke("create_test", { testData });
			}

			console.log("All data created successfully!");
		} catch (error) {
			console.error("Error creating data:", error);
		}
	}

	const superFormObj = superForm(defaults(zod4(newFgSchema)), {
		SPA: true,
		validators: zod4Client(newFgSchema),
		onUpdate({ form }) {
			if (form.valid) {
				console.log(form.data);
				submit_data(form.data);
			}
		}
	});

	const { form: formData, errors, message, constraints, enhance } = superFormObj;

	function addAttribute() {
		$formData.attributes = [...$formData.attributes, ""]
	}

	function addTest() {
		$formData.tests = [...$formData.tests, {
			type: '',
			frequency: 0,
			voltage: 0,
			min: 0,
			max: 0,
			output_uom: '',
			prim_pins: '',
			sec_pins: '',
			shorted_pins: '',
			description: ''
		}];
	}
</script>

<form method="POST" use:enhance>
	<Form.Field form={superFormObj} name="fg">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>FG</Form.Label>
				<Input {...props} bind:value={$formData.fg} />
			{/snippet}
		</Form.Control>
		<Form.Description />
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field form={superFormObj} name="rev">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Part Rev</Form.Label>
				<Input {...props} bind:value={$formData.rev} />
			{/snippet}
		</Form.Control>
		<Form.Description />
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field form={superFormObj} name="customer">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Part Customer</Form.Label>
				<Input {...props} bind:value={$formData.customer} />
			{/snippet}
		</Form.Control>
		<Form.Description />
		<Form.FieldErrors />
	</Form.Field>

	<Form.Fieldset form={superFormObj} name="attributes">
		<Form.Legend>Enter your attributes</Form.Legend>
		{#each $formData.attributes as _, i}
			<Form.ElementField form={superFormObj} name="attributes[{i}]">
				<Form.Control>
					{#snippet children({ props })}
						<Input type="attributes" bind:value={$formData.attributes[i]} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>
		{/each}
		<Form.Description>Your URLs will be displayed on your public profile.</Form.Description>
		<Form.FieldErrors />
	</Form.Fieldset>
	<Button onclick={addAttribute}>Add an Attribute</Button>

	<Form.Fieldset form={superFormObj} name="tests">
		<Form.Legend>Type</Form.Legend>
		{#each $formData.tests as _, i}
			<Form.ElementField form={superFormObj} name="tests[{i}].type">
				<Form.Legend>Enter your attributes</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="text" bind:value={$formData.tests[i].type} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].frequency">
				<Form.Legend>Frequency</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="number" bind:value={$formData.tests[i].frequency} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].voltage">
				<Form.Legend>Voltage</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="number" bind:value={$formData.tests[i].voltage} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].min">
				<Form.Legend>Minimum</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="number" bind:value={$formData.tests[i].min} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].max">
				<Form.Legend>Maximum</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="number" bind:value={$formData.tests[i].max} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].output_uom">
				<Form.Legend>Unit of Measurement</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="text" bind:value={$formData.tests[i].output_uom} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].prim_pins">
				<Form.Legend>Primary Pins</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="text" bind:value={$formData.tests[i].prim_pins} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].sec_pins">
				<Form.Legend>Secondary Pins</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="text" bind:value={$formData.tests[i].sec_pins} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>

			<Form.ElementField form={superFormObj} name="tests[{i}].shorted_pins">
				<Form.Legend>Shorted Pins</Form.Legend>
				<Form.Control>
					{#snippet children({ props })}
						<Input type="text" bind:value={$formData.tests[i].shorted_pins} {...props} />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.ElementField>
		{/each}
		<Form.Description>Your URLs will be displayed on your public profile.</Form.Description>
		<Form.FieldErrors />
	</Form.Fieldset>

	<Button onclick={addTest}>Add a Test</Button>

	<Button type="submit" class="mt-4 rounded border border-gray-300 px-4 py-2">Submit</Button>
</form>
