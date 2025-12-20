<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';


	let { data } = $props();
	// let reports = $state<ReportWithFGResponse[]>(data.reports);
	// let fgs = $state<FGResponse[]>(data.fgs);
	
</script>

<pre>{JSON.stringify(data.tests, null, 2)}</pre>