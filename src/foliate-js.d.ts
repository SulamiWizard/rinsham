// foliate-js ships no type declarations. This gives us just enough typing
// to use the <foliate-view> custom element without `any` errors.
declare module "foliate-js/view.js" {
  export interface RelocateDetail {
    /** EPUB CFI of the current location */
    cfi: string;
    /** Reading progress fraction (0–1) */
    fraction: number;
    location: { current: number; next: number; total: number };
    tocItem?: { label: string; href: string } | null;
    pageItem?: unknown;
  }

  /** The custom element registered by importing this module. */
  export class FoliateView extends HTMLElement {
    /** Open a book from a URL, File, Blob, or Book object. */
    open(book: string | File | Blob | object): Promise<void>;
    next(): Promise<void>;
    prev(): Promise<void>;
    goTo(target: string | number): Promise<void>;
    goLeft(): Promise<void>;
    goRight(): Promise<void>;
    close(): void;
    renderer: HTMLElement;
    book: unknown;
  }
}
