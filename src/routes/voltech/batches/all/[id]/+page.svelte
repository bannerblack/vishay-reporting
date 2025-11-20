<script lang="ts">
	import type { PageProps } from './$types';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import type { TestResult } from '$lib/db/database';
	import PackageIcon from '@lucide/svelte/icons/package';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import UserIcon from '@lucide/svelte/icons/user';
	import CheckCircleIcon from '@lucide/svelte/icons/check-circle';
	import XCircleIcon from '@lucide/svelte/icons/x-circle';
	import ArrowLeftIcon from '@lucide/svelte/icons/arrow-left';
	import FileTextIcon from '@lucide/svelte/icons/file-text';
	import LayersIcon from '@lucide/svelte/icons/layers';

	let { data }: PageProps = $props();
	let tests = $state<TestResult[]>(data.tests);
	let stats = $state(data.stats);
	let batchNumber = $state(data.batchNumber);

	function formatDate(dateStr: string): string {
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString();
		} catch {
			return dateStr;
		}
	}

	function formatDateTime(dateStr: string): string {
		try {
			const date = new Date(dateStr);
			return date.toLocaleString();
		} catch {
			return dateStr;
		}
	}

	function getPassRateColor(rate: number): string {
		if (rate >= 95) return 'text-green-600';
		if (rate >= 90) return 'text-yellow-600';
		return 'text-red-600';
	}
</script>

<div class="container mx-auto py-6 space-y-6 col-span-3">
	<!-- Header with Back Button -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-4">
			<Button variant="outline" size="sm" href="/voltech/batches">
				<ArrowLeftIcon class="mr-2 h-4 w-4" />
				Back to Batches
			</Button>
			<div>
				<h1 class="text-3xl font-bold">Batch {batchNumber} - All Tests</h1>
				<p class="text-muted-foreground">All test results across all dates for this batch</p>
			</div>
		</div>
	</div>

	<!-- Aggregate Stats -->
	<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Total Tests</Card.Title>
				<FileTextIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{stats.total_tests}</div>
				<p class="text-xs text-muted-foreground">
					<span class="text-green-600">{stats.passed}</span> passed, 
					<span class="text-red-600">{stats.failed}</span> failed
				</p>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Test Dates</Card.Title>
				<CalendarIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{stats.dates.length}</div>
				<p class="text-xs text-muted-foreground">
					{formatDate(stats.dates[0])} - {formatDate(stats.dates[stats.dates.length - 1])}
				</p>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Parts Tested</Card.Title>
				<PackageIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{stats.parts.length}</div>
				<p class="text-xs text-muted-foreground font-mono">
					{stats.parts.join(', ')}
				</p>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Operators</Card.Title>
				<UserIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{stats.operators.length}</div>
				<p class="text-xs text-muted-foreground">
					{stats.operators.join(', ')}
				</p>
			</Card.Content>
		</Card.Root>
	</div>

	<!-- Pass Rate Overview -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Overall Pass Rate</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<span class={getPassRateColor(stats.pass_rate) + ' text-3xl font-bold'}>
						{stats.pass_rate.toFixed(1)}%
					</span>
					{#if stats.pass_rate >= 95}
						<Badge variant="default" class="text-lg px-4 py-1">Excellent</Badge>
					{:else if stats.pass_rate >= 90}
						<Badge variant="secondary" class="text-lg px-4 py-1">Good</Badge>
					{:else}
						<Badge variant="destructive" class="text-lg px-4 py-1">Needs Attention</Badge>
					{/if}
				</div>
				<Progress value={stats.pass_rate} class="h-3" />
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Test Dates Breakdown -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<div>
					<Card.Title>Test Dates</Card.Title>
					<p class="text-sm text-muted-foreground mt-1">
						This batch was tested on {stats.dates.length} different {stats.dates.length === 1 ? 'date' : 'dates'}
					</p>
				</div>
			</div>
		</Card.Header>
		<Card.Content>
			<div class="flex flex-wrap gap-2">
				{#each stats.dates as date}
					<Badge variant="outline" class="text-sm">
						<CalendarIcon class="h-3 w-3 mr-1" />
						{formatDate(date)}
					</Badge>
				{/each}
			</div>
		</Card.Content>
	</Card.Root>

	<!-- All Test Results Table -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center justify-between">
				<Card.Title>All Test Results ({tests.length})</Card.Title>
				<Badge variant="secondary">
					<LayersIcon class="h-3 w-3 mr-1" />
					Across All Dates
				</Badge>
			</div>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Date</Table.Head>
						<Table.Head>Serial Number</Table.Head>
						<Table.Head>Part</Table.Head>
						<Table.Head>Operator</Table.Head>
						<Table.Head>Result</Table.Head>
						<Table.Head>Result #</Table.Head>
						<Table.Head>Timestamp</Table.Head>
						<Table.Head class="text-right">File</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if tests.length === 0}
						<Table.Row>
							<Table.Cell colspan={8} class="text-center text-muted-foreground py-8">
								No test results found for this batch.
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each tests as test (test.id)}
							<Table.Row class="hover:bg-muted/50">
								<Table.Cell class="text-sm">
									<Badge variant="outline" class="font-mono text-xs">
										{formatDate(test.date)}
									</Badge>
								</Table.Cell>
								<Table.Cell class="font-mono font-medium">
									{test.serial_num}
								</Table.Cell>
								<Table.Cell class="font-mono">
									{test.part}
								</Table.Cell>
								<Table.Cell class="text-sm">
									{test.operator}
								</Table.Cell>
								<Table.Cell>
									<div class="flex items-center gap-2">
										{#if test.pass_fail === 'PASS' || test.pass_fail === 'Pass'}
											<CheckCircleIcon class="h-4 w-4 text-green-600" />
											<Badge variant="default">Pass</Badge>
										{:else}
											<XCircleIcon class="h-4 w-4 text-red-600" />
											<Badge variant="destructive">Fail</Badge>
										{/if}
									</div>
								</Table.Cell>
								<Table.Cell class="font-mono text-sm text-muted-foreground">
									{test.result_num}
								</Table.Cell>
								<Table.Cell class="text-sm text-muted-foreground">
									{formatDateTime(test.created_at)}
								</Table.Cell>
								<Table.Cell class="text-right">
									<Button variant="ghost" size="sm" class="font-mono text-xs">
										{test.file_path.split('\\').pop()}
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
