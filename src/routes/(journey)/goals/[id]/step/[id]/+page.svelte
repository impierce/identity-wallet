<script lang="ts">
  import Button from '$src/lib/components/Button.svelte';
  import { goto } from '$app/navigation';
</script>

<div class="flex h-full flex-col items-center justify-center space-y-8 bg-neutral-100 p-8">
  <p class="rounded bg-orange-100 px-4 py-2 text-sm font-medium text-orange-600">
    TODO: send email
  </p>
  <input
    class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300 dark:caret-slate-300"
    placeholder="Enter your email"
  />
  <Button label="Send verification link" on:click={() => goto('/me')}></Button>
</div>
