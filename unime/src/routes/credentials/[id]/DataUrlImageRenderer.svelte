<script lang="ts">
  export let key: string;
  export let dataUrl: string;
</script>

<!--
@component
This renderer for `credentialSubject` fields renders an embedded image for a given [Data URL](https://developer.mozilla.org/en-US/docs/Web/URI/Schemes/data).
-->
<div class="flex flex-col items-start rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
  <h2 class="w-full font-medium text-text-alt">{key}</h2>
  <img src={dataUrl} alt="embedded" class="my-1 h-32 rounded-md object-contain" />
</div>
