<script lang="ts">
	import type { PageProps } from './$types';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import { voltech, type BatchListItem, type OverallStats } from '$lib/db/database';
	import PackageIcon from '@lucide/svelte/icons/package';
	import SearchIcon from '@lucide/svelte/icons/search';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import TrendingDownIcon from '@lucide/svelte/icons/trending-down';
	import ActivityIcon from '@lucide/svelte/icons/activity';
	import ArrowRightIcon from '@lucide/svelte/icons/arrow-right';
	import CalendarIcon from '@lucide/svelte/icons/calendar';

	let { data }: PageProps = $props();
	let batches = $state<BatchListItem[]>(data.recentBatches);
	let overallStats = $state<OverallStats | null>(data.overallStats);
	let searchQuery = $state('');
	let searching = $state(false);

	const filteredBatches = $derived(searchQuery.length >= 2 ? batches : data.recentBatches);

	async function handleSearch() {
		if (searchQuery.length < 2) {
			batches = data.recentBatches;
			return;
		}

		try {
			searching = true;
			// Search batches containing the query string
			const results = await voltech.searchBatches({
				limit: 50
			});
			
			// Filter by batch number containing search query
			batches = results.filter(batch => 
				batch.batch.toLowerCase().includes(searchQuery.toLowerCase())
			);
		} catch (error) {
			console.error('Search failed:', error);
		} finally {
			searching = false;
		}
	}

	function getPassRateColor(rate: number): string {
		if (rate >= 95) return 'text-green-600';
		if (rate >= 90) return 'text-yellow-600';
		return 'text-red-600';
	}

	function formatDate(dateStr: string): string {
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString();
		} catch {
			return dateStr;
		}
	}
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Batches</h1>
			<p class="text-muted-foreground">Browse and search batch test data</p>
		</div>
	</div>

	<!-- Overall Stats -->
	{#if overallStats}
		<div class="grid gap-4 md:grid-cols-4">
			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Total Batches</Card.Title>
					<PackageIcon class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">{overallStats.total_batches.toLocaleString()}</div>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Total Tests</Card.Title>
					<ActivityIcon class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">{overallStats.total_tests.toLocaleString()}</div>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Pass Rate</Card.Title>
					<TrendingUpIcon class="h-4 w-4 text-green-600" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">{overallStats.pass_rate.toFixed(1)}%</div>
					<p class="text-xs text-muted-foreground">
						{overallStats.passed.toLocaleString()} passed
					</p>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Failed Tests</Card.Title>
					<TrendingDownIcon class="h-4 w-4 text-red-600" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">{overallStats.failed.toLocaleString()}</div>
					<p class="text-xs text-muted-foreground">
						{(100 - overallStats.pass_rate).toFixed(1)}% fail rate
					</p>
				</Card.Content>
			</Card.Root>
		</div>
	{/if}

	<!-- Search and List -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center gap-4">
				<div class="relative flex-1">
					<SearchIcon class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Search batches..."
						bind:value={searchQuery}
						onkeydown={(e) => e.key === 'Enter' && handleSearch()}
						class="pl-8"
					/>
				</div>
				<Button onclick={handleSearch} disabled={searching || searchQuery.length < 2}>
					{searching ? 'Searching...' : 'Search'}
				</Button>
			</div>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Batch Number</Table.Head>
						<Table.Head>Part Number</Table.Head>
						<Table.Head>Date</Table.Head>
						<Table.Head>Operator</Table.Head>
						<Table.Head>Total Tests</Table.Head>
						<Table.Head>Pass Rate</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if filteredBatches.length === 0}
						<Table.Row>
							<Table.Cell colspan={7} class="text-center text-muted-foreground py-8">
								{searchQuery ? 'No batches match your search.' : 'No batches found.'}
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each filteredBatches as batch, index (batch.batch + '-' + batch.date + '-' + index)}
							<Table.Row class="hover:bg-muted/50">
								<Table.Cell class="font-mono font-medium">
									{batch.batch}
								</Table.Cell>
								<Table.Cell class="font-mono">
									{batch.part}
								</Table.Cell>
								<Table.Cell>
									<div class="flex items-center gap-2">
										<CalendarIcon class="h-3 w-3 text-muted-foreground" />
										{formatDate(batch.date)}
									</div>
								</Table.Cell>
								<Table.Cell>
									{batch.operator}
								</Table.Cell>
								<Table.Cell>
									<div class="space-y-1">
										<div>{batch.total_tests}</div>
										<div class="text-xs text-muted-foreground">
											<span class="text-green-600">{batch.passed}</span> / 
											<span class="text-red-600">{batch.failed}</span>
										</div>
									</div>
								</Table.Cell>
								<Table.Cell>
									<div class="space-y-1">
										<div class="flex items-center gap-2">
											<span class={getPassRateColor(batch.pass_rate)}>
												{batch.pass_rate.toFixed(1)}%
											</span>
											{#if batch.pass_rate >= 95}
												<Badge variant="default">Excellent</Badge>
											{:else if batch.pass_rate >= 90}
												<Badge variant="secondary">Good</Badge>
											{:else}
												<Badge variant="destructive">Needs Attention</Badge>
											{/if}
										</div>
										<Progress value={batch.pass_rate} class="h-1" />
									</div>
								</Table.Cell>
								<Table.Cell class="text-right">
									<div class="flex items-center justify-end gap-2">
										<Button variant="outline" size="sm" href={`/voltech/batches/${batch.batch}?date=${encodeURIComponent(batch.date)}`}>
											View Details
											<ArrowRightIcon class="ml-2 h-3 w-3" />
										</Button>
										<Button variant="secondary" size="sm" href={`/voltech/batches/all/${batch.batch}`}>
											All
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