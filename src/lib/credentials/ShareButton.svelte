<script lang="ts">
  import QrCode from '~icons/ph/qr-code';
  import LL from '$src/i18n/i18n-svelte';
  import { melt } from '@melt-ui/svelte';

  export let trigger: any = undefined; // TODO: add type
</script>

<button
  class="absolute bottom-4 right-4 flex h-12 items-center justify-center rounded-full bg-indigo-500 px-4 dark:bg-slate-800"
  use:melt={trigger}
>
  <QrCode class="h-6 w-6 text-white" />
  <p class="pl-2 text-[13px]/[24px] font-medium text-white">Share</p>
</button>
