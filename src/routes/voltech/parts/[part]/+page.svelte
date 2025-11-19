<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import { Separator } from '$lib/components/ui/separator';
	import type { PartSummary, BatchListItem, OverallStats } from '$lib/db/database';
	import PackageIcon from '@lucide/svelte/icons/package';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import TrendingDownIcon from '@lucide/svelte/icons/trending-down';
	import ActivityIcon from '@lucide/svelte/icons/activity';
	import UsersIcon from '@lucide/svelte/icons/users';
	import ArrowLeftIcon from '@lucide/svelte/icons/arrow-left';
	import BarChart3Icon from '@lucide/svelte/icons/bar-chart-3';

	let { data } = $props();
	let summary = $state<PartSummary>(data.summary);
	let batches = $state<string[]>(data.batches);
	let recentBatches = $state<BatchListItem[]>(data.recentBatches);
	let stats = $state<OverallStats | null>(data.stats);

	function getPassRateColor(rate: number): string {
		if (rate >= 95) return 'text-green-600';
		if (rate >= 90) return 'text-yellow-600';
		return 'text-red-600';
	}
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<!-- Header -->
	<div class="flex items-center gap-4">
		<Button variant="outline" size="icon" href="/voltech/parts">
			<ArrowLeftIcon class="h-4 w-4" />
		</Button>
		<div>
			<h1 class="text-3xl font-bold font-mono">{data.part}</h1>
			<p class="text-muted-foreground">Part test history and statistics</p>
		</div>
	</div>

	<!-- Summary Stats -->
	<div class="grid gap-4 md:grid-cols-3 lg:grid-cols-5">
		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Total Batches</Card.Title>
				<PackageIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{summary.total_batches.toLocaleString()}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Total Tests</Card.Title>
				<ActivityIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{summary.total_tests.toLocaleString()}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Passed</Card.Title>
				<TrendingUpIcon class="h-4 w-4 text-green-600" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold text-green-600">{summary.passed.toLocaleString()}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Failed</Card.Title>
				<TrendingDownIcon class="h-4 w-4 text-red-600" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold text-red-600">{summary.failed.toLocaleString()}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Pass Rate</Card.Title>
				<BarChart3Icon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold {getPassRateColor(summary.pass_rate)}">
					{summary.pass_rate.toFixed(1)}%
				</div>
				<Progress value={summary.pass_rate} class="mt-2 h-1" />
			</Card.Content>
		</Card.Root>
	</div>

	<!-- Date Range -->
	<Card.Root>
		<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<CalendarIcon class="h-5 w-5" />
				Testing Period
			</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="flex items-center gap-4">
				<div>
					<div class="text-sm text-muted-foreground">First Test</div>
					<div class="text-lg font-medium">{new Date(summary.first_date).toLocaleDateString()}</div>
				</div>
				<div class="text-muted-foreground">→</div>
				<div>
					<div class="text-sm text-muted-foreground">Last Test</div>
					<div class="text-lg font-medium">{new Date(summary.last_date).toLocaleDateString()}</div>
				</div>
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Recent Batches -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Recent Batches</Card.Title>
			<p class="text-sm text-muted-foreground">
				Last 10 batches tested for this part
			</p>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Batch</Table.Head>
						<Table.Head>Date</Table.Head>
						<Table.Head>Operator</Table.Head>
						<Table.Head>Tests</Table.Head>
						<Table.Head>Passed</Table.Head>
						<Table.Head>Failed</Table.Head>
						<Table.Head>Pass Rate</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if recentBatches.length === 0}
						<Table.Row>
							<Table.Cell colspan={8} class="text-center text-muted-foreground py-8">
								No batch data available
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each recentBatches as batch (batch.batch + '-' + batch.date)}
							<Table.Row class="hover:bg-muted/50">
								<Table.Cell class="font-mono">
									{batch.batch}
								</Table.Cell>
								<Table.Cell>
								{new Date(batch.date).toLocaleDateString()}
							</Table.Cell>
							<Table.Cell>
								<div class="flex items-center gap-1 text-xs text-muted-foreground">
									<UsersIcon class="h-3 w-3 text-muted-foreground" />
									<code class="text-xs">{batch.operator}</code>
								</div>
							</Table.Cell>
								<Table.Cell>
									{batch.total_tests}
								</Table.Cell>
								<Table.Cell>
									<span class="text-green-600">{batch.passed}</span>
								</Table.Cell>
								<Table.Cell>
									<span class="text-red-600">{batch.failed}</span>
								</Table.Cell>
								<Table.Cell>
									<div class="space-y-1">
										<span class={getPassRateColor(batch.pass_rate)}>
											{batch.pass_rate.toFixed(1)}%
										</span>
										<Progress value={batch.pass_rate} class="h-1" />
									</div>
								</Table.Cell>
								<Table.Cell class="text-right">
									<Button variant="outline" size="sm" href={`/voltech/batches/${batch.batch}`}>
										View
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
