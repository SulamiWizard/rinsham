<script lang="ts">
  import LibraryEntry from "$lib/components/LibraryEntry.svelte";
  import type { ScannedFile } from "$lib/types";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { invoke } from "@tauri-apps/api/core";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";

  let books: ScannedFile[] = $state([]);

  // The library directory is the source of truth, so always reload from it
  // rather than appending — that way a book can never show up twice.
  async function loadLibrary(): Promise<void> {
    books = await invoke<ScannedFile[]>("scan_epubs");
  }

  // Let the user pick one or more directories, then hand each off to the
  // Rust `import_epubs` command which copies new .epub files into the library.
  async function scanDir(): Promise<void> {
    const selected = await openDialog({
      multiple: true,
      directory: true,
    });
    if (!selected) return; // user cancelled

    const dirs = Array.isArray(selected) ? selected : [selected];
    for (const dir of dirs) {
      await invoke<number>("import_epubs", { dir, recursive: true });
    }
    await loadLibrary();
  }

  function openBook(book: ScannedFile): void {
    // resolve() only accepts route paths, not query strings, so the lint rule
    // can't see that this URL is already base-path aware.
    // eslint-disable-next-line svelte/no-navigation-without-resolve
    goto(`${resolve("/reader")}?path=${encodeURIComponent(book.path)}`);
  }

  onMount(loadLibrary);
</script>

<!-- Entry page: Library -->

<!-- TODO: grid of library entries -->
<div class="TEST">
  <button onclick={() => scanDir()}>Open Dialogue</button>
  <button onclick={() => (books = [])}>Clear Library</button>
</div>
<div class="Library-grid">
  {#each books as book (book.path)}
    <button class="entry-button" onclick={() => openBook(book)}>
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
