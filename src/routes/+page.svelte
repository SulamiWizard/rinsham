<script lang="ts">
  import LibraryEntry from "$lib/components/LibraryEntry.svelte";
  import type { ScannedFile } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";

  let books: ScannedFile[] = $state([]);

  // Let the user pick one or more directories, then hand each off to the
  // Rust `scan_epubs` command which recursively walks for .epub files.
  async function scanDir(): Promise<void> {
    const selected = await openDialog({
      multiple: true,
      directory: true,
    });
    if (!selected) return; // user cancelled

    const dirs = Array.isArray(selected) ? selected : [selected];
    for (const dir of dirs) {
      const found = await invoke<ScannedFile[]>("scan_epubs", {
        dir,
        recursive: true,
      });
      books.push(...found);
    }
  }
</script>

<!-- Entry page: Library -->

<!-- TODO: grid of library entries -->
<div class="TEST">
  <button onclick={() => scanDir()}>Open Dialogue</button>
  <button onclick={() => (books = [])}>Clear Library</button>
</div>
<div class="Library-grid">
  {#each books as book (book.path)}
    <button class="entry-button" onclick={() => console.log(book.path)}>
      <LibraryEntry entry={book} />
      TODO!
    </button>
  {/each}
</div>

<style>
  .entry-button {
    width: auto;
  }

  .Library-grid {
    display: grid;
    grid-template-columns: auto auto auto;
    /* background-color: dodgerblue; */
    padding: 10px;
  }
</style>
