<script lang="ts">
  export let value: number;
</script>

<!-- TODO: remove unused classes -->

<div class="flex h-[10px] overflow-hidden rounded bg-grey dark:bg-blue">
  <div
    style="width:{value}%"
    class="flex flex-col justify-center whitespace-nowrap rounded-lg bg-primary text-center text-white shadow-none"
  />
</div>
