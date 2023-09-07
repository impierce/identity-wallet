<script lang="ts">
  import { Button, Input } from '@impierce/ui-components';
</script>

<div class="flex h-full flex-col items-center justify-center space-y-8 bg-neutral-100 p-8">
  <p class="rounded bg-orange-100 px-4 py-2 text-sm font-medium text-orange-600">
    TODO: send email
  </p>
  <Input type="text" placeholder="Enter your email" class="w-fit" />
  <Button href="/me">Send verification link</Button>
</div>
