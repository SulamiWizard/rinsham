<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { FoliateView, RelocateDetail } from "foliate-js/view.js";

  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { readFile } from "@tauri-apps/plugin-fs";

  let viewerContainer: HTMLDivElement;
  let viewElement: FoliateView | undefined;

  // Hardcoded for testing, TODO: fix
  // export let epubUrl = "/test-data/book.epub"; // file would be in rinsham/static/test-data/book.epub
  let error: string = "";
  let bookTitle: string = "";

  async function pickEpub(): Promise<File | null> {
    const selected = await openDialog({
      multiple: false,
      filters: [{ name: "EPUB", extensions: ["epub"] }],
    });
    if (!selected) return null; // user cancelled

    const path = Array.isArray(selected) ? selected[0]! : selected;
    // Android silently ignores the filter above (no MIME entry for "epub"),
    // so the picker there is effectively unfiltered — guard client-side.
    if (!path.toLowerCase().endsWith(".epub")) return null;

    const bytes = await readFile(path); // resolves content:// on Android, plain path elsewhere
    const name = path.split(/[\\/]/).pop() ?? "book.epub";
    return new File([bytes], name, { type: "application/epub+zip" });
  }
  async function openBook() {
    error = "";
    try {
      const file = await pickEpub();
      if (!file) return; // user cancelled
      await viewElement?.open(file);
      viewElement?.goTo(2);
      bookTitle = file.name;
    } catch (e) {
      console.error("Failed to open EPUB:", e);
      error = e instanceof Error ? e.message : "Failed to open file";
    }
  }

  onMount(async () => {
    await import("foliate-js/view.js");
    viewElement = document.createElement("foliate-view") as FoliateView;
    viewerContainer.appendChild(viewElement);

    viewElement.addEventListener("relocate", (e) => {
      const detail = (e as CustomEvent<RelocateDetail>).detail;
      console.log("Location changed:", detail);
    });
    viewElement.addEventListener("load", (e) => {
      const { doc } = (e as CustomEvent<{ doc: Document; index: number }>)
        .detail;
      doc.addEventListener("pointerdown", handleZoneTap);
    });
  });

  function handleZoneTap(event: PointerEvent) {
    // Ignore taps while the user is selecting text
    const sel = (event.target as Element)?.ownerDocument?.getSelection?.();
    if (sel && !sel.isCollapsed) return;

    // TODO: allow user to click on links
    if ((event.target as Element).closest("a")) {
      return;
    }

    const width = (event.currentTarget as Document).defaultView!.innerWidth;
    const x = event.clientX; // relative to the iframe's viewport

    if (x < width * 0.3) {
      prevPage();
    } else if (x > width * 0.7) {
      nextPage();
    }
    // middle 40% left as a no-op / future menu zone
  }

  // Cleanup on component destruction. onMount can only return a *synchronous*
  // cleanup function, so with an async callback we use onDestroy instead.
  onDestroy(() => {
    viewElement?.remove();
  });

  // Example navigation controls
  function nextPage() {
    viewElement?.next();
  }

  function prevPage() {
    viewElement?.prev();
  }
</script>

<div class="reader-wrapper">
  <!-- Navigation Controls -->
  <div class="toolbar">
    <button on:click={openBook}>Open Book</button>
    <button on:click={prevPage}>Previous</button>
    <button on:click={nextPage}>Next</button>
  </div>

  <!-- The container element where foliate-view will attach -->
  <div bind:this={viewerContainer} class="viewer-container"></div>
</div>

<style>
  .reader-wrapper {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100%;
  }
  .toolbar {
    padding: 10px;
    background: #f5f5f5;
    display: flex;
    gap: 10px;
  }
  .viewer-container {
    flex: 1;
    width: 100%;
    height: 100%;
    position: relative;
  }
  /* Ensure the custom element fills the container */
  :global(foliate-view) {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>
