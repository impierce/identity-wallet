<script>
  import { page } from '$app/stores';
</script>

<div class="flex h-screen flex-col items-center justify-center p-4">
  <div class="bg-red-100 rounded p-4">
    <h2 class="font-mono text-red-600 text-sm">{$page.error.message}</h2>
    <div class="font-mono text-red-800 opacity-50 font-semibold text-sm pt-4">{$page.error.errorId}</div>
  </div>
</div>
