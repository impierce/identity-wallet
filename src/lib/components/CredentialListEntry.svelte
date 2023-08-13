<script lang="ts">
  import { goto } from '$app/navigation';

  export let title = '';
  export let description = '';
  export let color: string | undefined = undefined;
</script>

<button
  class="flex h-[60px] items-center justify-start rounded-xl bg-white p-[5px] pr-[24px]"
  on:click={() => goto('/credentials/0')}
>
  <!-- Icon -->
  <div
    class={`mr-[15px] ${color} flex h-[50px] w-[50px] flex-col items-center justify-center rounded-lg`}
  >
    <slot name="icon" />
  </div>
  <!-- Text -->
  <div class="flex flex-col items-start">
    <p class="text-[13px]/[24px] font-medium text-slate-800">{title}</p>
    <p class="text-[12px]/[20px] font-medium text-slate-400">{description}</p>
  </div>
</button>
