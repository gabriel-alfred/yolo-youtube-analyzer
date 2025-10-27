<script lang="ts">
  import Layout from '$lib/components/layout/Layout.svelte';
  import "../app.css"
  
  interface Props {
    children?: import('svelte').Snippet;
  }
  
  let { children }: Props = $props();
</script>

<Layout>
  {#snippet children()}
    {#if children}
      {@render children()}
    {/if}
  {/snippet}
</Layout>