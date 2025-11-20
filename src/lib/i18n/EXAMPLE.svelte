<!-- Example: Using i18n strings in a Svelte component -->
<script lang="ts">
	import { tr } from '$lib/i18n/strings';
	
	// Usage 1: Direct function call
	const title = tr('batches.title');
	
	// Usage 2: In template (most common)
	// See the template below
</script>

<div>
	<h1>{tr('batches.title')}</h1>
	<p>{tr('batches.subtitle')}</p>
	
	<button>{tr('common.search')}</button>
	
	<!-- Dynamic strings with variables -->
	<p>{tr('batches.passed')}: 1</p>
	
	<!-- Conditional strings -->
	{#if true}
		<p>{tr('common.loading')}</p>
	{/if}
</div>
