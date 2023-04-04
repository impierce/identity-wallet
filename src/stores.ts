import { readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { navigate } from "svelte-navigator";
import type { TransferState as State } from '../src-tauri/bindings/TransferState'; // TODO: copy to /src instead of importing across root

export const state = readable<State>(undefined, (set) => {
  listen('state-changed', (event) => {
    console.log(event);
    set(event.payload as State);
    console.log((event.payload as State))
    if ((event.payload as State).active_profile === null) {
      navigate("welcome")
    } else {
      navigate("profile")
    }
  });
  // TODO: unsubscribe from listener?
});
