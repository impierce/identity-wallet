<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';

  import { tintFor } from './ecosystem';
  import EcosystemAvatar from './EcosystemAvatar.svelte';

  export let ecosystem: EcosystemProfile;
  export let isTempAsset = true;

  // A trust anchor that names only itself leads its own ecosystem; repeating its name says nothing.
  $: leader = ecosystem.ecosystem_leader.name !== ecosystem.name ? ecosystem.ecosystem_leader : undefined;
</script>

<!--
@component
The top of an ecosystem's detail page: its logo and name, the leader below the name, and the member count.

### Props
- ecosystem
- isTempAsset (_default_: `true`)
-->
<div class="flex flex-col items-center gap-3 rounded-2xl {tintFor(ecosystem.name)} px-4 py-6">
  <EcosystemAvatar
    name={ecosystem.name}
    logoUri={ecosystem.logo_uri}
    class="size-[75px] rounded-2xl bg-white/50"
    textClass="text-[22px]/[30px]"
    {isTempAsset}
  />
  <div class="flex flex-col items-center gap-1">
    <p class="text-center text-[20px]/[28px] font-semibold">{ecosystem.name}</p>
    {#if leader}
      <div class="flex items-center gap-1.5">
        <EcosystemAvatar
          name={leader.name}
          logoUri={leader.logo_uri}
          class="size-5 rounded-full bg-white/50"
          textClass="text-[8px]/[10px]"
          {isTempAsset}
        />
        <p class="text-[13px]/[20px] font-medium">
          {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_LED_BY({ name: leader.name })}
        </p>
      </div>
    {/if}
  </div>
  <span class="rounded-md bg-white/50 px-2 py-1 text-[12px]/[18px] font-medium">
    {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_MEMBERS({ count: ecosystem.member_count })}
  </span>
</div>
