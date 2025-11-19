<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import type { OverallStats, DailyStats, OperatorStats, TestResult } from '$lib/db/database';
	import ActivityIcon from '@lucide/svelte/icons/activity';
	import PackageIcon from '@lucide/svelte/icons/package';
	import UsersIcon from '@lucide/svelte/icons/users';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import TrendingDownIcon from '@lucide/svelte/icons/trending-down';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import AlertCircleIcon from '@lucide/svelte/icons/alert-circle';
	import BarChart3Icon from '@lucide/svelte/icons/bar-chart-3';

	let { data } = $props();
	let overallStats = $state<OverallStats | null>(data.overallStats);
	let dailyStats = $state<DailyStats[]>(data.dailyStats);
	let operatorStats = $state<OperatorStats[]>(data.operatorStats);
	let failedTests = $state<TestResult[]>(data.failedTests);

	function getPassRateColor(rate: number): string {
		if (rate >= 95) return 'text-green-600';
		if (rate >= 90) return 'text-yellow-600';
		return 'text-red-600';
	}

	const topOperators = $derived(
		operatorStats.slice().sort((a, b) => b.total_tests - a.total_tests).slice(0, 5)
	);

	const recentDays = $derived(
		dailyStats.slice().sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime()).slice(0, 7)
	);
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<!-- Header -->
	<div>
		<h1 class="text-3xl font-bold">Voltech Statistics</h1>
		<p class="text-muted-foreground">Test performance metrics and analytics</p>
	</div>

	<!-- Overall Stats -->
	{#if overallStats}
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
			<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Total Tests</Card.Title>
				<ActivityIcon class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">{overallStats.total_tests.toLocaleString()}</div>
					<p class="text-xs text-muted-foreground mt-1">
						Across {overallStats.total_parts} parts
					</p>
				</Card.Content>
			</Card.Root>

			<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Pass Rate</Card.Title>
				<TrendingUpIcon class="h-4 w-4 text-green-600" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold text-green-600">
						{overallStats.pass_rate.toFixed(1)}%
					</div>
					<Progress value={overallStats.pass_rate} class="mt-2" />
				</Card.Content>
			</Card.Root>

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
				<Card.Title class="text-sm font-medium">Operators</Card.Title>
				<UsersIcon class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">{overallStats.total_operators}</div>
				</Card.Content>
			</Card.Root>
		</div>
	{/if}

	<div class="grid gap-6 lg:grid-cols-2">
		<!-- Recent Daily Stats -->
		<Card.Root>
			<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<CalendarIcon class="h-5 w-5" />
				Last 7 Days
				</Card.Title>
			</Card.Header>
			<Card.Content>
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Date</Table.Head>
							<Table.Head>Tests</Table.Head>
							<Table.Head>Pass Rate</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each recentDays as day (day.date)}
							<Table.Row>
								<Table.Cell>
									{new Date(day.date).toLocaleDateString()}
								</Table.Cell>
								<Table.Cell>
									<div class="flex items-center gap-2">
										<span>{day.total_tests}</span>
										<span class="text-xs text-muted-foreground">
											({day.total_parts} parts)
										</span>
									</div>
								</Table.Cell>
								<Table.Cell>
									<div class="space-y-1">
										<span class={getPassRateColor(day.pass_rate)}>
											{day.pass_rate.toFixed(1)}%
										</span>
										<Progress value={day.pass_rate} class="h-1" />
									</div>
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</Card.Content>
		</Card.Root>

		<!-- Top Operators -->
		<Card.Root>
			<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<BarChart3Icon class="h-5 w-5" />
				Top Operators
				</Card.Title>
			</Card.Header>
			<Card.Content>
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Operator</Table.Head>
							<Table.Head>Tests</Table.Head>
							<Table.Head>Pass Rate</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each topOperators as operator (operator.operator)}
							<Table.Row>
								<Table.Cell>
									<code class="text-sm">{operator.operator}</code>
								</Table.Cell>
								<Table.Cell>
									<div class="flex flex-col gap-1">
										<span>{operator.total_tests.toLocaleString()}</span>
										<span class="text-xs text-muted-foreground">
											{operator.parts_tested} parts, {operator.batches_completed} batches
										</span>
									</div>
								</Table.Cell>
								<Table.Cell>
									<div class="space-y-1">
										<span class={getPassRateColor(operator.pass_rate)}>
											{operator.pass_rate.toFixed(1)}%
										</span>
										<Progress value={operator.pass_rate} class="h-1" />
									</div>
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</Card.Content>
		</Card.Root>
	</div>

	<!-- Recent Failed Tests -->
	<Card.Root>
		<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<AlertCircleIcon class="h-5 w-5 text-destructive" />
				Recent Failed Tests
			</Card.Title>
			<p class="text-sm text-muted-foreground">
				Last 20 failed tests across all parts
			</p>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Part</Table.Head>
						<Table.Head>Batch</Table.Head>
						<Table.Head>Serial</Table.Head>
						<Table.Head>Test Name</Table.Head>
						<Table.Head>Date</Table.Head>
						<Table.Head>Operator</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if failedTests.length === 0}
						<Table.Row>
							<Table.Cell colspan={6} class="text-center text-muted-foreground py-8">
								No failed tests found
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each failedTests as test (test.id)}
							<Table.Row class="hover:bg-muted/50">
								<Table.Cell class="font-mono">
									<Button variant="link" href={`/voltech/parts/${test.part}`} class="p-0 h-auto">
										{test.part}
									</Button>
								</Table.Cell>
								<Table.Cell class="font-mono text-sm">
									{test.batch}
								</Table.Cell>
								<Table.Cell class="font-mono text-sm">
									{test.serial_num}
								</Table.Cell>
								<Table.Cell>
									<span class="text-sm">{test.test_name}</span>
								</Table.Cell>
								<Table.Cell>
									<span class="text-sm text-muted-foreground">
										{new Date(test.created_at).toLocaleDateString()}
									</span>
								</Table.Cell>
								<Table.Cell>
									<code class="text-xs">{test.operator}</code>
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</Card.Content>
	</Card.Root>
</div>
