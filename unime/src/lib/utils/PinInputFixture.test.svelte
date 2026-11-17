<!-- A PIN input set up as on the credential offer prompt, for `pinInput.test.ts`. -->
<script lang="ts">
  import { PinInput } from 'melt/builders';

  import { fillPinInput } from './pinInput';

  const { type = 'numeric', maxLength = 6 }: { type?: PinInput['type']; maxLength?: number } = $props();

  const pinInput = new PinInput({
    type: () => type,
    maxLength: () => maxLength,
    placeholder: '',
    onPaste: (text) => fillPinInput(pinInput, text),
  });
</script>

<div {...pinInput.root}>
  {#each pinInput.inputs as input}
    <input {...input} />
  {/each}
</div>
<button disabled={pinInput.value.length !== maxLength}>Accept</button>
