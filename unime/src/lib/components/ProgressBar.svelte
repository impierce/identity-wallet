<script lang="ts">
  export let value: number;
</script>

<!-- TODO: remove unused classes -->

<div class="bg-grey dark:bg-blue flex h-[10px] overflow-hidden rounded-sm">
  <div
    style="width:{value}%"
    class="bg-primary flex flex-col justify-center whitespace-nowrap rounded-lg text-center text-white shadow-none"
  ></div>
</div>
