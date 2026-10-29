<script lang="ts">
  import { error as logError } from '@tauri-apps/plugin-log';
  import { openUrl } from '@tauri-apps/plugin-opener';

  import { isBrowserUrl } from '$lib/utils/url';

  export let href: string;

  let className = '';
  export { className as class };

  async function open() {
    if (!isBrowserUrl(href)) return;
    try {
      await openUrl(href);
    } catch (e) {
      logError(`Failed to open URL ${href}: ${e}`);
    }
  }
</script>

<!--
@component
A link that opens `href` in the system browser, on desktop as well as on mobile. Only `http(s)` URLs are opened.
-->
<a {href} target="_blank" rel="noopener noreferrer" class={className} on:click|preventDefault|stopPropagation={open}>
  <slot />
</a>
