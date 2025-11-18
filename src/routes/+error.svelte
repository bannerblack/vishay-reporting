<script lang="ts">
    import { page } from '$app/state';
</script>

<h1>There's been a problem!</h1>
<h1>{page.status}: {page.error?.message}</h1>