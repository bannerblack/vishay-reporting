<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Check, ChevronsUpDown } from '@lucide/svelte';
	import { cn } from '$lib/utils';

	interface Item {
		value: string | number;
		label: string;
	}

	interface Props {
		items: Item[];
		value?: string | number | null;
		onSelect: (value: string | number) => void;
		placeholder?: string;
		searchPlaceholder?: string;
		emptyMessage?: string;
		class?: string;
	}

	let {
		items,
		value = $bindable(),
		onSelect,
		placeholder = 'Select item...',
		searchPlaceholder = 'Search...',
		emptyMessage = 'No item found.',
		class: className
	}: Props = $props();

	let open = $state(false);

	const selectedItem = $derived(
		items.find((item) => item.value === value)
	);
</script>

<Popover.Root bind:open>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button
				{...props}
				variant="outline"
				role="combobox"
				aria-expanded={open}
				class={cn('w-full justify-between', className)}
			>
				<span class="truncate">
					{selectedItem ? selectedItem.label : placeholder}
				</span>
				<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-[400px] p-0">
		<Command.Root>
			<Command.Input placeholder={searchPlaceholder} />
			<Command.Empty>{emptyMessage}</Command.Empty>
			<Command.Group class="max-h-[300px] overflow-auto">
				{#each items as item (item.value)}
					<Command.Item
						value={item.label}
						onSelect={() => {
							onSelect(item.value);
							open = false;
						}}
					>
						<Check
							class={cn(
								'mr-2 h-4 w-4',
								value === item.value ? 'opacity-100' : 'opacity-0'
							)}
						/>
						{item.label}
					</Command.Item>
				{/each}
			</Command.Group>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
