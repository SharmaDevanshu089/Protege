<script>
  import TitleBar from '$lib/Titlebar.svelte';
</script>

<TitleBar />

<main style="padding-top: 32px;">
  <slot />
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: sans-serif;
  }
</style>