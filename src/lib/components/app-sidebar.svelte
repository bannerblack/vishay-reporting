<script lang="ts">
	import NavMain from "./nav-main.svelte";
	import NavProjects from "./nav-projects.svelte";
	import NavSecondary from "./nav-secondary.svelte";
	import NavUser from "./nav-user.svelte";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import CommandIcon from "@lucide/svelte/icons/command";
	import PackageIcon from "@lucide/svelte/icons/package";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import BeakerIcon from "@lucide/svelte/icons/beaker";
	import LockIcon from "@lucide/svelte/icons/lock";
	import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
	import Settings2Icon from "@lucide/svelte/icons/settings-2";
	import DatabaseIcon from "@lucide/svelte/icons/database";
	import BarChart3Icon from "@lucide/svelte/icons/bar-chart-3";
	import type { ComponentProps } from "svelte";
	import { getUserContext } from '$lib/context/user-context.svelte';

	let { ref = $bindable(null), ...restProps }: ComponentProps<typeof Sidebar.Root> = $props();

	const userContext = getUserContext();

	// Determine navigation items based on permissions
	const canAccessManage = $derived(userContext.hasAnyPermission(['pe', 'qa', 'admin']));
	const canAccessUsers = $derived(userContext.hasPermission('admin'));

	const navMain = $derived([
		...(canAccessManage ? [{
			title: "Manage",
			url: "/",
			icon: PackageIcon,
			isActive: true,
			items: [
				{
					title: "Finished Goods",
					url: "/manage/fg",
				},
				{
					title: "Reports",
					url: "/manage/report",
				},
				{
					title: "Tests",
					url: "/manage/test",
				},
				{
					title: "Events",
					url: "/event",
				},
				{
					title: "New Events",
					url: "/event/new",
				},
				...(canAccessUsers ? [{
					title: "Users",
					url: "/manage/user",
				}] : []),
			],
		}] : []),
		{
			title: "Voltech",
			url: "/voltech/parts",
			icon: DatabaseIcon,
			items: [
				{
					title: "Parts",
					url: "/voltech/parts",
				},
				{
					title: "Statistics",
					url: "/voltech/stats",
				},
				{
					title: "Batches",
					url: "/voltech/batches",
				},
				...(canAccessManage ? [{
					title: "Management",
					url: "/manage/voltech",
				}, {
					title: "Full Import",
					url: "/manage/voltech/import",
				}] : []),
			],
		},
		{
			title: "Reports",
			url: "/report",
			icon: FileTextIcon,
			items: [
				{
					title: "All Reports",
					url: "/report",
				},
			],
		},
		{
			title: "Tests",
			url: "/test",
			icon: BeakerIcon,
			items: [
				{
					title: "All Tests",
					url: "/test",
				},
			],
		},
		{
			title: "Locked Items",
			url: "/locked",
			icon: LockIcon,
			items: [
				{
					title: "View Locked",
					url: "/locked",
				},
			],
		},
		{
			title: "Returns",
			url: "/return",
			icon: ArrowLeftIcon,
			items: [
				{
					title: "Process Returns",
					url: "/return",
				},
			],
		},
	]);

	const navSecondary = [
		{
			title: "Settings",
			url: "/settings",
			icon: Settings2Icon,
		},
	];

	const userData = $derived({
		name: userContext.user?.name || "Guest",
		email: `@${userContext.user?.username || "guest"}`,
		avatar: "",
	});
</script>

<Sidebar.Root bind:ref variant="inset" {...restProps}>
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton size="lg">
					{#snippet child({ props })}
						<a href="/" {...props}>
							<div
								class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
							>
								<CommandIcon class="size-4" />
							</div>
							<div class="grid flex-1 text-left text-sm leading-tight">
								<span class="truncate font-medium">Vishay Testing</span>
								<span class="truncate text-xs">Quality Control</span>
							</div>
						</a>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>
	<Sidebar.Content>
		<NavMain items={navMain} />
		<NavSecondary items={navSecondary} class="mt-auto" />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser user={userData} />
	</Sidebar.Footer>
</Sidebar.Root>
