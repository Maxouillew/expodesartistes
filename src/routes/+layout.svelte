<script lang="ts">
  import "$lib/styles/global.scss";
  import { onMount, type Snippet } from "svelte";
  import { restoreFullscreen } from "$lib/services/window";

  let { children }: { children: Snippet } = $props();

  onMount(restoreFullscreen);
</script>

{@render children()}
