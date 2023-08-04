<script lang="ts">
  import { Button, Input } from '@impierce/ui-components';
</script>

<div class="flex items-center justify-center flex-col h-full space-y-8 p-8">
  <p class="text-sm rounded bg-orange-100 px-4 py-2 font-medium text-orange-600">
    TODO: send email
  </p>
  <Input type="text" placeholder="Enter your email" class="w-fit" />
  <Button href="/me">Send verification link</Button>
</div>
