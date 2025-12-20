<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import * as Select from '$lib/components/ui/select';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '$lib/components/ui/label';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Save } from '@lucide/svelte';
	import type { TestData } from '$lib/db/adapters/test';
	import { testTypes, type AssociatedTests } from '$lib/db/database';
	import { onMount } from 'svelte';
	import { map } from 'zod';
	import test from 'node:test';

	interface Props {
		onSubmit: (testData: Omit<TestData, 'added_by'>) => Promise<void>;
		onCancel?: () => void;
		reportId?: number;
		fgId: number;
		fgNumber?: string;
		buttonText?: string;
	}

	let {
		onSubmit,
		onCancel,
		reportId,
		fgId,
		fgNumber,
		buttonText = 'Create Test'
	}: Props = $props();

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
	let testTypeOptions = $state<[string, string, string][]>([]);
	let associatedTests = $state<AssociatedTests>([]);
	let isLoadingTests = $state(false);
	let selectedTestType = $state<string | undefined>(undefined);

	// Example grouping by source_type (3rd element) - reactive derived so it updates when testTypeOptions changes
	const grouped_tests = $derived(() => {
		const acc: Record<string, { name: string; description: string }[]> = {};
		for (const item of testTypeOptions) {
			const [name, description, group] = item as unknown as [string, string, string];
			const key = (group || 'Other').toString();
			if (!acc[key]) acc[key] = [];
			acc[key].push({ name, description });
		}
		return acc;
	});

	onMount(async () => {
		try {
			testTypeOptions = await testTypes.getTestTypes();
		} catch (error) {
			console.error('Failed to load test types:', error);
		}
	});

	// Reactive effect: When test type changes, fetch associated tests
	$effect(() => {
		if (selectedTestType && fgNumber) {
			loadAssociatedTests(selectedTestType, fgNumber);
		} else {
			associatedTests = [];
		}
	});

	async function loadAssociatedTests(testType: string, fg: string) {
		isLoadingTests = true;
		try {
			associatedTests = await testTypes.findTestsForType(fg, testType);
			console.log(associatedTests);
		} catch (error) {
			console.error('Failed to load associated tests:', error);
			associatedTests = [];
		} finally {
			isLoadingTests = false;
		}
	}

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
		selectedTestType = undefined;
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

	// Get available tests based on source type
	const availableTests = $derived(() => {
		const source = (formData.source_type || '').toLowerCase();
		if (source.includes('voltech')) {
			return associatedTests ?? [];
		} else if (source.includes('manual')) {
			return associatedTests ?? [];
		}
		return [];
	});
</script>

<!-- <pre>{JSON.stringify(formData.source_type, null, 2)}</pre>
<pre>{JSON.stringify(testTypeOptions, null, 2)}</pre>

<pre>{JSON.stringify(grouped_tests(), null, 2)}</pre> -->

<!-- Render grouped tests for debugging / visual verification -->
{#each Object.keys(grouped_tests()) as group}
	<div class="mt-2">
		<h4 class="font-semibold">{group}</h4>
		<ul class="ml-4 list-disc">
			{#each grouped_tests()[group] as item}
				<li>{item.name} — <span class="text-xs text-muted-foreground">{item.description}</span></li>
			{/each}
		</ul>
	</div>
{/each}

{#if false}
	<!-- no-op placeholder for future 'group selection' control (currently not used) -->
{/if}

{#if formData.source_type && formData.source_type.toLowerCase().includes('voltech')}
	Voltech
{:else if formData.source_type && formData.source_type.toLowerCase().includes('manual')}
	Manual Electrical
{/if}

<div class="space-y-4 rounded-lg border bg-muted/50 p-4">
	<h3 class="font-semibold">Create New Test</h3>
	<div class="grid grid-cols-2 gap-4">
		<!-- Source Type Selection -->
		<div class="grid gap-2">
			<Label for="source_type_select">Data Source</Label>
			<Select.Root
				type="single"
				value={formData.source_type}
				onValueChange={(value) => {
					formData.source_type = value || 'other';
					// Reset selected test type and associated tests when source changes
					selectedTestType = undefined;
					formData.test_type = '';
					associatedTests = [];
				}}
			>
				<Select.Trigger id="source_type_select">
					{formData.source_type.charAt(0).toUpperCase() + formData.source_type.slice(1)}
				</Select.Trigger>
				<Select.Content>
					<!-- Use group keys as Source Type options -->
					{#each Object.keys(grouped_tests()) as option}
						<Select.Item value={option}>{option}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Test Type Selection -->
		<div class="grid gap-2">
			<Label for="test_type_select">Test Type Category</Label>
			<Select.Root
				type="single"
				value={selectedTestType}
				onValueChange={(value) => {
					selectedTestType = value;
					formData.test_type = value || '';
				}}
			>
				<Select.Trigger id="test_type_select">
					{selectedTestType || 'Select test type...'}
				</Select.Trigger>
				<Select.Content>
					{#each grouped_tests()[formData.source_type] ?? [] as item}
						<Select.Item value={item.name}>{item.name}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Associated Test Selection (only for voltech/manual) -->
		{#if formData.source_type !== 'other' && selectedTestType}
			<div class="col-span-2 grid gap-2">
				<Label for="associated_test">
					Associated Test
					{#if isLoadingTests}
						<span class="text-xs text-muted-foreground">(Loading...)</span>
					{/if}
				</Label>
				<pre>{JSON.stringify(availableTests(), null, 2)}</pre>
				<Select.Root
					type="single"
					value={formData.associated_test || undefined}
					onValueChange={(value) => {
						formData.associated_test = value || null;
					}}
					disabled={isLoadingTests || (availableTests()?.length ?? 0) === 0}
				>
					<Select.Trigger id="associated_test">
						{formData.associated_test ||
							((availableTests()?.length ?? 0) === 0 ? 'No tests found' : 'Select associated test...')}
					</Select.Trigger>
					<Select.Content>
						{#if associatedTests.length > 0}
							{#each associatedTests as item}
								<Select.Item value={item}>{item}</Select.Item>
							{/each}
						{:else if associatedTests?.length > 0}
							{#each associatedTests as item}
								<Select.Item value={item}>{item}</Select.Item>
							{/each}
						{/if}
					</Select.Content>
				</Select.Root>
			</div>
		{/if}

		<!-- Manual Override Checkbox (only for 'other' source) -->
		{#if formData.source_type === 'other'}
			<div class="col-span-2 flex items-center space-x-2">
				<Checkbox
					id="manual_override"
					checked={formData.manual_override ?? false}
					onCheckedChange={(checked) => {
						formData.manual_override = checked === true ? true : null;
					}}
				/>
				<Label for="manual_override" class="text-sm font-normal">
					Manual override (use custom test specifications)
				</Label>
			</div>
		{/if}

		<div class="grid gap-2">
			<Label for="uo_m">Unit of Measure *</Label>
			<Input id="uo_m" bind:value={formData.uo_m} required />
		</div>
		<div class="grid gap-2">
			<Label for="frequency">Frequency</Label>
			<Input id="frequency" type="number" bind:value={formData.frequency} step="0.01" />
		</div>
		<div class="grid gap-2">
			<Label for="voltage">Voltage</Label>
			<Input id="voltage" type="number" bind:value={formData.voltage} step="0.01" />
		</div>
		<div class="grid gap-2">
			<Label for="minimum">Minimum</Label>
			<Input id="minimum" type="number" bind:value={formData.minimum} step="0.01" />
		</div>
		<div class="grid gap-2">
			<Label for="maximum">Maximum</Label>
			<Input id="maximum" type="number" bind:value={formData.maximum} step="0.01" />
		</div>
		<div class="grid gap-2">
			<Label for="order">Order</Label>
			<Input id="order" type="number" bind:value={formData.order} />
		</div>
		<div class="grid gap-2">
			<Label for="primary_pins">Primary Pins</Label>
			<Input id="primary_pins" bind:value={formData.primary_pins} />
		</div>
		<div class="grid gap-2">
			<Label for="secondary_pins">Secondary Pins</Label>
			<Input id="secondary_pins" bind:value={formData.secondary_pins} />
		</div>
		<div class="col-span-2 grid gap-2">
			<Label for="shorted_pins">Shorted Pins</Label>
			<Input id="shorted_pins" bind:value={formData.shorted_pins} />
		</div>
		<div class="col-span-2 grid gap-2">
			<Label for="description">Description</Label>
			<Textarea id="description" bind:value={formData.description} rows={3} />
		</div>
	</div>
	<div class="flex gap-2">
		<Button onclick={handleSubmit} disabled={isSubmitting}>
			<Save class="mr-2 h-4 w-4" />
			{buttonText}
		</Button>
		{#if onCancel}
			<Button variant="outline" onclick={onCancel} disabled={isSubmitting}>Cancel</Button>
		{/if}
	</div>
</div>
