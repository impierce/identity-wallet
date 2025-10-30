<script lang="ts">
  const { initials, picture }: { initials?: string; picture?: string | null } = $props();
</script>

<div class="bg-primary flex size-11 cursor-default items-center justify-center rounded-2xl">
  {#if picture}
    <span class="text-[28px]/[28px]">
      <!-- The profile picture is an emoticon selected from a list of emoticons we provide. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html picture}
    </span>
  {:else}
    <span class="dark:text-dark text-[20px]/[20px] font-semibold text-white">
      {initials}
    </span>
  {/if}
</div>
