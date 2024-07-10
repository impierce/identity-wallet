import 'unplugin-icons/types/svelte';

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // Return type of `handleError` hook.
    interface Error {
      message: string;
      errorId: string;
    }
    // interface Locals {}
    // interface PageData {}
    interface PageState {
      tab?: string;
    }
    // interface Platform {}
  }
}

export {};
