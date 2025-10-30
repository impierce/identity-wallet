<script lang="ts">
  export let key: string;
  export let value: string;
</script>

<div class="bg-background rounded-xl px-4 py-3 text-[13px]/[24px]">
  <h2 class="text-text-alt font-medium">{key}</h2>
  <p class="overflow-x-auto">{value}</p>
</div>
