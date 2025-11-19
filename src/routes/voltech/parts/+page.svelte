<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import { voltech, type PartListItem, type PartSummary, type OverallStats } from '$lib/db/database';
	import PackageIcon from '@lucide/svelte/icons/package';
	import SearchIcon from '@lucide/svelte/icons/search';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import TrendingDownIcon from '@lucide/svelte/icons/trending-down';
	import ActivityIcon from '@lucide/svelte/icons/activity';
	import ArrowRightIcon from '@lucide/svelte/icons/arrow-right';

	let { data } = $props();
	let parts = $state<PartListItem[]>(data.parts);
	let overallStats = $state<OverallStats | null>(data.overallStats);
	let searchQuery = $state('');
	let searching = $state(false);

	const filteredParts = $derived(
		searchQuery.length >= 2 ? parts : data.parts
	);

	async function handleSearch() {
		if (searchQuery.length < 2) {
			parts = data.parts;
			return;
		}

		try {
			searching = true;
			const results = await voltech.searchParts(searchQuery, 50);
			
			// Get details for each part
			const partDetails = await Promise.all(
				results.map(async (partNum) => {
					const summary = await voltech.getPartSummary(partNum);
					if (summary) {
						return {
							part: summary.part,
							total_tests: summary.total_tests,
							passed: summary.passed,
							failed: summary.failed,
							pass_rate: summary.pass_rate
						};
					}
					return null;
				})
			);
			
			parts = partDetails.filter(p => p !== null) as PartListItem[];
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
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Parts</h1>
			<p class="text-muted-foreground">Browse and search part test data</p>
		</div>
	</div>

	<!-- Overall Stats -->
	{#if overallStats}
		<div class="grid gap-4 md:grid-cols-4">
			<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Total Parts</Card.Title>
				<PackageIcon class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">{overallStats.total_parts.toLocaleString()}</div>
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
						placeholder="Search parts..."
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
						<Table.Head>Part Number</Table.Head>
						<Table.Head>Total Tests</Table.Head>
						<Table.Head>Passed</Table.Head>
						<Table.Head>Failed</Table.Head>
						<Table.Head>Pass Rate</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if filteredParts.length === 0}
						<Table.Row>
							<Table.Cell colspan={6} class="text-center text-muted-foreground py-8">
								{searchQuery ? 'No parts match your search.' : 'No parts found.'}
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each filteredParts as part (part.part)}
							<Table.Row class="hover:bg-muted/50">
								<Table.Cell class="font-mono font-medium">
									{part.part}
								</Table.Cell>
								<Table.Cell>
									{part.total_tests.toLocaleString()}
								</Table.Cell>
								<Table.Cell>
									<span class="text-green-600">{part.passed.toLocaleString()}</span>
								</Table.Cell>
								<Table.Cell>
									<span class="text-red-600">{part.failed.toLocaleString()}</span>
								</Table.Cell>
								<Table.Cell>
									<div class="space-y-1">
										<div class="flex items-center gap-2">
											<span class={getPassRateColor(part.pass_rate)}>
												{part.pass_rate.toFixed(1)}%
											</span>
											{#if part.pass_rate >= 95}
												<Badge variant="default">Excellent</Badge>
											{:else if part.pass_rate >= 90}
												<Badge variant="secondary">Good</Badge>
											{:else}
												<Badge variant="destructive">Needs Attention</Badge>
											{/if}
										</div>
										<Progress value={part.pass_rate} class="h-1" />
									</div>
								</Table.Cell>
								<Table.Cell class="text-right">
									<Button variant="outline" size="sm" href={`/voltech/parts/${part.part}`}>
									View Details
									<ArrowRightIcon class="ml-2 h-3 w-3" />
									</Button>
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</Card.Content>
	</Card.Root>
</div>
