<script lang="ts">
  export let key: string;
  export let value: string;
</script>

<div class="rounded-xl bg-background-alt px-4 py-3 text-[13px]/[24px]">
  <h2 class="font-medium text-text-alt">{key}</h2>
  <p class="overflow-x-auto">{value}</p>
</div>
