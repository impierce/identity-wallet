<script lang="ts">
  import { Combobox } from 'melt/builders';

  // Uses the Iconify API instead of Svelte components
  import Icon from '@iconify/svelte';

  import { CaretDownBoldIcon, CheckBoldIcon, GlobeRegularIcon } from '$lib/icons';

  let { value = $bindable() }: { value?: string } = $props();

  const options = [
    { code: 'DE', name: 'Deutschland' },
    { code: 'NL', name: 'Nederland' },
    { code: 'SE', name: 'Sverige' },
    { code: 'ES', name: 'España' },
    { code: 'GB', name: 'United Kingdom' },
  ] as const;

  const names = options.map((o) => o.name);

  type Option = (typeof names)[number];

  const combobox = new Combobox<Option>({
    onValueChange: (val) => {
      value = options.find((o) => o.name === val)?.code;
    },
  });

  const filtered = $derived.by(() => {
    if (!combobox.touched) return names;
    return names.filter((o) => o.toLowerCase().includes(combobox.inputValue.trim().toLowerCase()));
  });
</script>

<div>
  <div class="relative text-left transition">
    <label for={combobox.ids.input} class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">Country</label>
    <div class="relative">
      <div class="absolute left-3 top-1/2 -translate-y-1/2">
        {#if combobox.value}
          <Icon
            class="size-5"
            icon={`circle-flags:${options.find((o) => o.name === combobox.value)?.code.toLowerCase()}`}
          />
        {:else}
          <GlobeRegularIcon class="-ml-[2px] -mt-[1px] size-6 text-slate-500 dark:text-slate-300" />
        {/if}
      </div>
      <input
        {...combobox.input}
        class="w-full rounded-xl border border-slate-300 bg-background-alt px-10 py-3 text-[14px]/[22px] font-medium text-slate-800 dark:border-slate-600 dark:text-grey"
        placeholder="Choose a country"
      />
      <button
        {...combobox.trigger}
        class="absolute right-1 top-1/2 grid size-10 -translate-y-1/2 place-items-center rounded-lg hover:bg-teal-50"
      >
        <CaretDownBoldIcon class="size-5 text-primary" />
      </button>
    </div>
  </div>

  <div
    {...combobox.content}
    class="m-0 max-h-40 rounded-xl border border-slate-300 bg-background-alt p-2 dark:border-slate-600"
  >
    {#each filtered as option (option)}
      <div {...combobox.getOption(option)} class="flex items-center rounded-lg p-2 hover:bg-teal-100">
        <Icon class="mr-2 size-5" icon={`circle-flags:${options.find((o) => o.name === option)?.code.toLowerCase()}`} />
        <div class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">{option}</div>
        {#if combobox.isSelected(option)}
          <CheckBoldIcon class="size-5 text-primary" />
        {/if}
      </div>
    {:else}
      <span class="text-[13px]/[24px] font-medium text-slate-500 p-2">No results found</span>
    {/each}
  </div>
</div>
