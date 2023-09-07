<script lang="ts">
  export let value: number;
</script>

<!-- TODO: remove unused classes -->

<div class="flex h-[10px] overflow-hidden rounded bg-[#E8E9F1] dark:bg-bg-dark-secondary">
  <div
    style="width:{value}%"
    class="flex flex-col justify-center whitespace-nowrap rounded-lg bg-primary text-center text-white shadow-none"
  />
</div>
