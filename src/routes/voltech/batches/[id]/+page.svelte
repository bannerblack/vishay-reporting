<script lang="ts">
	import type { PageProps } from './$types';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import type { TestResult, BatchSummary } from '$lib/db/database';
	import PackageIcon from '@lucide/svelte/icons/package';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import UserIcon from '@lucide/svelte/icons/user';
	import CheckCircleIcon from '@lucide/svelte/icons/check-circle';
	import XCircleIcon from '@lucide/svelte/icons/x-circle';
	import ArrowLeftIcon from '@lucide/svelte/icons/arrow-left';
	import FileTextIcon from '@lucide/svelte/icons/file-text';

	let { data }: PageProps = $props();
	let batch = $state<BatchSummary>(data.batch);
	let tests = $state<TestResult[]>(data.tests);
	let dateFilter = $state<string | null>(data.dateFilter ?? null);

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
				<h1 class="text-3xl font-bold">Batch {batch.batch}</h1>
				<p class="text-muted-foreground">
					{#if dateFilter}
						Test results for {formatDate(dateFilter)}
					{:else}
						All test results for this batch
					{/if}
				</p>
			</div>
		</div>
		{#if dateFilter}
			<Button variant="secondary" size="sm" href={`/voltech/batches/all/${batch.batch}`}>
				View All Tests
			</Button>
		{/if}
	</div>

	<!-- Batch Summary Stats -->
	<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Part Number</Card.Title>
				<PackageIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold font-mono">{batch.part}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Date</Card.Title>
				<CalendarIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{formatDate(batch.date)}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Operator</Card.Title>
				<UserIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{batch.operator}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
				<Card.Title class="text-sm font-medium">Total Tests</Card.Title>
				<FileTextIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content>
				<div class="text-2xl font-bold">{batch.total_tests}</div>
				<p class="text-xs text-muted-foreground">
					<span class="text-green-600">{batch.passed}</span> passed, 
					<span class="text-red-600">{batch.failed}</span> failed
				</p>
			</Card.Content>
		</Card.Root>
	</div>

	<!-- Pass Rate Overview -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Pass Rate</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<span class={getPassRateColor(batch.pass_rate) + ' text-3xl font-bold'}>
						{batch.pass_rate.toFixed(1)}%
					</span>
					{#if batch.pass_rate >= 95}
						<Badge variant="default" class="text-lg px-4 py-1">Excellent</Badge>
					{:else if batch.pass_rate >= 90}
						<Badge variant="secondary" class="text-lg px-4 py-1">Good</Badge>
					{:else}
						<Badge variant="destructive" class="text-lg px-4 py-1">Needs Attention</Badge>
					{/if}
				</div>
				<Progress value={batch.pass_rate} class="h-3" />
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Test Results Table -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Test Results ({tests.length})</Card.Title>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Serial Number</Table.Head>
						<Table.Head>Test Name</Table.Head>
						<Table.Head>Result</Table.Head>
						<Table.Head>Result #</Table.Head>
						<Table.Head>Timestamp</Table.Head>
						<Table.Head class="text-right">File</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if tests.length === 0}
						<Table.Row>
							<Table.Cell colspan={6} class="text-center text-muted-foreground py-8">
								No test results found for this batch.
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each tests as test (test.id)}
							<Table.Row class="hover:bg-muted/50">
								<Table.Cell class="font-mono font-medium">
									{test.serial_num}
								</Table.Cell>
								<Table.Cell>
									{test.part}
								</Table.Cell>
								<Table.Cell>
									<div class="flex items-center gap-2">
										{#if test.pass_fail === 'Pass'}
											<Badge variant="default" class="w-14"><CheckCircleIcon class="h-4 w-4 text-green-600" /> Pass</Badge>
										{:else}
											<Badge variant="destructive" class="w-14"><XCircleIcon class="h-4 w-4 text-black-600" /> Fail</Badge>
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