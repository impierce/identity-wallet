<script lang="ts">
  import EnvelopeSimple from '~icons/ph/envelope-simple-light';
  import House from '~icons/ph/house-light';

  import CredentialListEntry from '../components/CredentialListEntry.svelte';
</script>

<div class="flex flex-col space-y-4">
  <CredentialListEntry
    title="Email address"
    description="ferris.rustacean@example.org"
    color="bg-purple-100"
  >
    <span slot="icon"><EnvelopeSimple class="h-6 w-6" /></span>
  </CredentialListEntry>
  <CredentialListEntry
    title="Home address"
    description="Sunset Rd 1337, Iron Islands"
    color="bg-green-100"
  >
    <span slot="icon"><House class="h-6 w-6" /></span>
  </CredentialListEntry>
</div>
