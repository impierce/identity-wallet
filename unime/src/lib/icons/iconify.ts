import circleFlags from '@iconify-json/circle-flags/icons.json';
import flagpack from '@iconify-json/flagpack/icons.json';
import { addCollection } from '@iconify/svelte';

// Add icon collections manually to Iconify so they can be loaded
// dynamically during runtime and are not stripped out by the bundler.
export function loadIcons(): void {
  addCollection(circleFlags);
  addCollection(flagpack);
}
