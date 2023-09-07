<script lang="ts">
  import { goto } from '$app/navigation';

  export let id: string | undefined = undefined; // TODO: should not be able to be undefined
  export let title = '';
  export let description = '';
  export let color: string | undefined = undefined;
</script>

<button
  class="flex h-[64px] items-center justify-start rounded-xl bg-bg-primary p-[7px] pr-[24px] dark:bg-bg-dark-primary"
  on:click={() => goto(`/credentials/${id}`)}
>
  <!-- Icon -->
  <div
    class="mr-[15px] {color} flex h-[50px] w-[50px] flex-col items-center justify-center rounded-lg"
  >
    <slot name="icon" />
    <slot name="logo" />
  </div>
  <!-- Logo (alternative to Icon) -->
  <!-- Text -->
  <div class="flex grow flex-col items-start">
    <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-white">{title}</p>
    <p class="max-w-[180px] truncate text-[12px]/[20px] font-medium text-slate-400">
      {description}
    </p>
  </div>
</button>
