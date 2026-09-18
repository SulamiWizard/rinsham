<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { FoliateView, RelocateDetail } from "foliate-js/view.js";

  let viewerContainer: HTMLDivElement;
  let viewElement: FoliateView | undefined;

  // Hardcoded for testing, TODO: fix
  export let epubUrl = "/test-data/book.epub"; // file would be in rinsham/static/test-data/book.epub

  onMount(async () => {
    // Dynamically import foliate-js to keep it strictly client-side.
    // Importing registers the <foliate-view> custom element.
    await import("foliate-js/view.js");

    // Create the custom web component element
    viewElement = document.createElement("foliate-view") as FoliateView;
    viewerContainer.appendChild(viewElement);

    // Set up event listeners for navigation and progress tracking
    viewElement.addEventListener("relocate", (e) => {
      const detail = (e as CustomEvent<RelocateDetail>).detail;
      console.log("Location changed:", detail);
      // detail contains trackable data like CFIs and progress percentages
    });

    try {
      // Load the book
      await viewElement.open(epubUrl);
    } catch (error) {
      console.error("Failed to load EPUB:", error);
    }
  });

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
