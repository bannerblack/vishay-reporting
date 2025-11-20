<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { Save } from '@lucide/svelte';
	import type { TestData } from '$lib/db/adapters/test';

	interface Props {
		onSubmit: (testData: Omit<TestData, 'added_by'>) => Promise<void>;
		onCancel?: () => void;
		reportId?: number;
		fgId: number;
		buttonText?: string;
	}

	let { onSubmit, onCancel, reportId, fgId, buttonText = 'Create Test' }: Props = $props();

	let formData = $state({
		test_type: '',
		frequency: null as number | null,
		voltage: null as number | null,
		minimum: null as number | null,
		maximum: null as number | null,
		uo_m: '',
		primary_pins: '',
		secondary_pins: '',
		shorted_pins: '',
		description: '',
		order: 0,
		source_type: 'other',
		associated_test: null as string | null,
		manual_override: null as boolean | null
	});

	let isSubmitting = $state(false);

	function resetForm() {
		formData = {
			test_type: '',
			frequency: null,
			voltage: null,
			minimum: null,
			maximum: null,
			uo_m: '',
			primary_pins: '',
			secondary_pins: '',
			shorted_pins: '',
			description: '',
			order: 0,
			source_type: 'other',
			associated_test: null,
			manual_override: null
		};
	}

	async function handleSubmit() {
		if (isSubmitting) return;
		
		try {
			isSubmitting = true;
			const testData: Omit<TestData, 'added_by'> = {
				report_id: reportId ?? null,
				fg_id: fgId,
				test_type: formData.test_type,
				frequency: formData.frequency,
				voltage: formData.voltage,
				minimum: formData.minimum,
				maximum: formData.maximum,
				uo_m: formData.uo_m,
				primary_pins: formData.primary_pins || null,
				secondary_pins: formData.secondary_pins || null,
				shorted_pins: formData.shorted_pins || null,
				description: formData.description || null,
				order: formData.order,
				source_type: formData.source_type,
				associated_test: formData.associated_test,
				manual_override: formData.manual_override
			};

			await onSubmit(testData);
			resetForm();
		} catch (error) {
			console.error('Error submitting test:', error);
			throw error;
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="border rounded-lg p-4 space-y-4 bg-muted/50">
	<h3 class="font-semibold">Create New Test</h3>
	<div class="grid grid-cols-2 gap-4">
		<div class="grid gap-2">
			<label for="test_type" class="text-sm font-medium">Test Type *</label>
			<Input id="test_type" bind:value={formData.test_type} required />
		</div>
		<div class="grid gap-2">
			<label for="uo_m" class="text-sm font-medium">Unit of Measure *</label>
			<Input id="uo_m" bind:value={formData.uo_m} required />
		</div>
		<div class="grid gap-2">
			<label for="frequency" class="text-sm font-medium">Frequency</label>
			<Input id="frequency" type="number" bind:value={formData.frequency} step="0.01" />
		</div>
		<div class="grid gap-2">
			<label for="voltage" class="text-sm font-medium">Voltage</label>
			<Input id="voltage" type="number" bind:value={formData.voltage} step="0.01" />
		</div>
		<div class="grid gap-2">
			<label for="minimum" class="text-sm font-medium">Minimum</label>
			<Input id="minimum" type="number" bind:value={formData.minimum} step="0.01" />
		</div>
		<div class="grid gap-2">
			<label for="maximum" class="text-sm font-medium">Maximum</label>
			<Input id="maximum" type="number" bind:value={formData.maximum} step="0.01" />
		</div>
		<div class="grid gap-2">
			<label for="primary_pins" class="text-sm font-medium">Primary Pins</label>
			<Input id="primary_pins" bind:value={formData.primary_pins} />
		</div>
		<div class="grid gap-2">
			<label for="secondary_pins" class="text-sm font-medium">Secondary Pins</label>
			<Input id="secondary_pins" bind:value={formData.secondary_pins} />
		</div>
		<div class="grid gap-2 col-span-2">
			<label for="shorted_pins" class="text-sm font-medium">Shorted Pins</label>
			<Input id="shorted_pins" bind:value={formData.shorted_pins} />
		</div>
		<div class="grid gap-2 col-span-2">
			<label for="description" class="text-sm font-medium">Description</label>
			<Textarea id="description" bind:value={formData.description} rows={3} />
		</div>
	</div>
	<div class="flex gap-2">
		<Button onclick={handleSubmit} disabled={isSubmitting}>
			<Save class="mr-2 h-4 w-4" />
			{buttonText}
		</Button>
		{#if onCancel}
			<Button variant="outline" onclick={onCancel} disabled={isSubmitting}>
				Cancel
			</Button>
		{/if}
	</div>
</div>
