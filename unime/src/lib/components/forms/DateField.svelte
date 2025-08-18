<script lang="ts">
  import { DateField } from 'bits-ui';

  import { CalendarDate } from '@internationalized/date';

  let value = $state(new CalendarDate(1987, 5, 12));

  const { label } = $props();
</script>

<DateField.Root bind:value>
  <div class="flex w-full flex-col gap-1">
    <DateField.Label class="block select-none text-sm font-medium">{label}</DateField.Label>
    <DateField.Input
      class="flex h-[50px] w-full select-none items-center rounded-xl border border-slate-300 bg-background-alt px-3 py-3 text-[13px]/[24px] tracking-[0.01em] text-slate-800 dark:border-slate-600 dark:text-grey"
    >
      {#snippet children({ segments })}
        {#each segments as { part, value }, i (part + i)}
          <div class="inline-block select-none">
            {#if part === 'literal'}
              <DateField.Segment {part} class="p-1 text-text-alt">
                {value}
              </DateField.Segment>
            {:else}
              <DateField.Segment
                {part}
                class="aria-[valuetext=Empty]:text-muted-foreground data-invalid:text-destructive rounded-[5px] px-1 py-1 text-text focus:bg-slate-200 dark:focus:bg-navy"
              >
                {value}
              </DateField.Segment>
            {/if}
          </div>
        {/each}
      {/snippet}
    </DateField.Input>
  </div>
</DateField.Root>
