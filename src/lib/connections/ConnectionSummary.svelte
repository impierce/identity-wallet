<script lang="ts">
  export let id: string;
  export let domain: string;
  export let displayName: string | undefined = undefined;
  export let lastConnected: string;
</script>

<div>
  <!-- ConnectionDetails -->
  <!-- <div>{id}</div> -->
    <div>{displayName}</div>
    <div>{domain}</div>
    <div>first connected: unknown</div>
    <div>{lastConnected}</div>
</div>
