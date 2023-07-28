<script>
  import { page } from '$app/stores';
  import { Button } from '@impierce/ui-components';
</script>

<div class="flex h-screen flex-col items-center justify-center p-4">
  <div class="rounded bg-red-100 p-4">
    <h2 class="font-mono text-sm text-red-600">{$page.error.message}</h2>
    <div class="pt-4 font-mono text-sm font-semibold text-red-800 opacity-50">
      {$page.error.errorId}
    </div>
  </div>
  <Button href="/me" variant="ghost" class="m-4">Go back</Button>
</div>
