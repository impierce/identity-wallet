<script lang="ts">
  import { CredentialListEntry } from '@impierce/ui-components';
  import MailCheck from '~icons/lucide/mail-check';
  import Home from '~icons/lucide/home';
</script>

<div class="flex flex-col space-y-4">
  <CredentialListEntry title="Email address" description="ferris.rustacean@example.org">
    <span slot="icon"><MailCheck class="h-6 w-6 text-violet-500" /></span>
  </CredentialListEntry>
  <CredentialListEntry title="Home address" description="Sunset Rd 1337, Iron Islands">
    <span slot="icon"><Home class="h-6 w-6 text-violet-500" /></span>
  </CredentialListEntry>
</div>
