<script lang="ts">
  import LibraryEntry from "$lib/components/LibraryEntry.svelte";
  import type { ScannedFile } from "$lib/types";
  import { basename } from "@tauri-apps/api/path";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { open } from "@tauri-apps/plugin-fs";

  // TODO: use real books from scan instead of fake placeholders
  const book1: ScannedFile = {
    path: "book1.epub",
    name: "Book 1",
    size: 1024,
  };
  const book2: ScannedFile = {
    path: "book2.epub",
    name: "Book 2",
    size: 1024,
  };

  let books: ScannedFile[] = $state([book1, book2]);

  // TODO: file browser to choose directory and scan

  async function scanDir(): Promise<string[] | null> {
    const selected = await openDialog({
      multiple: true,
      directory: true,
    });
    console.log(selected);
    // TODO: recursive walk through dir to find all the books
    if (!selected) return null; // user cancelled
    for (const file of selected) {
      let book = await scanFile(file);
      if (book) {
        books.push(book);
      }
    }
    return selected;
  }

  async function scanFile(path: string): Promise<ScannedFile | null> {
    // if file is not epub return null
    if (!path.endsWith(".epub")) {
      return null;
    }

    // scan file and return ScannedFile struct/interface
    const file = await open(path);
    const fileInfo = await file.stat();
    const book: ScannedFile = {
      path: path,
      name: await basename(path),
      size: fileInfo.size,
    };

    return book;
  }
</script>

<!-- Entry page: Library -->

<!-- TODO: grid of library entries -->
<div class="TEST"><button onclick={() => scanDir()}>OPEN DIALOGUE</button></div>
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
