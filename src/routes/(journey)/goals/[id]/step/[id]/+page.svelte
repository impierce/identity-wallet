<script lang="ts">
  import { Button, Input } from '@impierce/ui-components';
</script>

<div class="flex items-center justify-center flex-col h-full space-y-8 p-8">
  <Input type="text" placeholder="Enter your email" class="w-fit" />
  <Button href="/me">Send verification link</Button>
</div>
