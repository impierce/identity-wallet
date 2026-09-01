<script lang="ts">
  import { calculateOrgInitials, tintFor } from './ecosystem';
  import LogoBadge from './LogoBadge.svelte';

  // Loose props rather than an `EcosystemProfile`: `Member` has the same two fields, so one
  // component marks ecosystems, leaders and members alike.
  export let name: string;
  export let logoUri: string | null = null;

  let className = '';
  export { className as class };
  export let textClass = 'text-[18px]/[24px]';

  $: initials = calculateOrgInitials(name);
  $: tint = tintFor(name).badge;
</script>

<!--
@component
The mark of an ecosystem or one of its members.

### Props
- name
- logoUri (optional)
- class (_sizing and rounding_)
- textClass (_default_: `'text-[18px]/[24px]'`)
-->
<LogoBadge {logoUri} {initials} {tint} class={className} {textClass} />
