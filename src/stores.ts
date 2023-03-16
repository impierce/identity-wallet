import { readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import type { TransferState as State } from '../src-tauri/bindings/TransferState'; // TODO: copy to /src instead of importing across root

export const state = readable<State>(undefined, (set) => {
  listen('state-changed', (event) => {
    console.log(event);
    set(event.payload as State);
  });
  // TODO: unsubscribe from listener?
});
