<script lang="ts">
  import type { ScannedFile } from "$lib/types";

  let { entry }: { entry: ScannedFile } = $props();
</script>

<div class="entry">
  <!-- <img {src} alt="Cover for {title}" /> -->
  <p class="title">{entry.name}</p>
  <p class="path">{entry.path}</p>
  <p class="size">{entry.size}</p>
</div>

<style>
  .entry {
    border: 2px solid black;
  }
  p {
    text-align: center;
  }
</style>
