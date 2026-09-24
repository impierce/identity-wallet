<script lang="ts">
  import type { Member } from '@bindings/user_prompt/Member';

  import EcosystemAvatar from './EcosystemAvatar.svelte';

  export let member: Member;
  export let isTempAsset = true;

  // Shown as sent: bare hosts reach this field too, so `hostname()` would drop what it cannot
  // parse. It carries no validation result, hence no shield.
</script>

<!--
@component
One organisation inside an ecosystem. Not a link — members have no page of their own.

### Props
- member
-->
<div class="flex w-full items-center py-3">
  <EcosystemAvatar
    name={member.name}
    logoUri={member.logo_uri}
    class="mr-4 size-10 rounded-full"
    textClass="text-[13px]/[18px]"
    {isTempAsset}
  />

  <div class="flex min-w-0 grow flex-col">
    <p class="truncate text-[13px]/[20px] font-medium text-slate-800 dark:text-grey">
      {member.name}
    </p>
    {#if member.description}
      <p class="truncate text-[12px]/[20px] font-normal text-slate-500 dark:text-slate-300">
        {member.description}
      </p>
    {/if}
    {#if member.identifier}
      <p class="truncate text-[12px]/[20px] font-normal text-primary">
        {member.identifier}
      </p>
    {/if}
  </div>
</div>
