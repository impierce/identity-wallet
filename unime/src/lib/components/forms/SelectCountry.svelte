<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { Combobox } from 'melt/builders';

  // Uses the Iconify API instead of Svelte components to allow dynamic imports
  import Icon from '@iconify/svelte';

  import countries from '$lib/components/forms/countries';
  import { CaretDownBoldIcon, CheckBoldIcon, GlobeRegularIcon } from '$lib/icons';

  interface Props {
    label?: string;
    placeholder?: string;
    value?: string;
  }

  let { label, placeholder, value = $bindable() }: Props = $props();

  const names = countries.map((o) => o.name);

  type Option = (typeof names)[number];

  const combobox = new Combobox<Option>({
    value: () => {
      return countries.find((c) => c.code === value)?.name;
    },
    onValueChange: (val) => {
      value = countries.find((c) => c.name === val)?.code;
    },
  });

  const filtered = $derived.by(() => {
    if (!combobox.touched) return names;
    return names.filter((o) => o.toLowerCase().includes(combobox.inputValue.trim().toLowerCase()));
  });
</script>

<div class="w-full">
  <div class="relative text-left transition">
    <label for={combobox.ids.input} class="dark:text-grey text-[14px]/[22px] font-medium text-slate-800">
      {label || $LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_COUNTRY_LABEL()}
    </label>
    <div class="relative">
      <div class="absolute left-3 top-1/2 -translate-y-1/2">
        {#if combobox.value}
          <Icon
            class="size-5"
            icon={`circle-flags:${countries.find((c) => c.name === combobox.value)?.code.toLowerCase()}`}
          />
        {:else}
          <!-- Margins are fine-tuned to align the icon shape with the circle-flags -->
          <GlobeRegularIcon class="-ml-[2px] -mt-px size-6 text-slate-500 dark:text-slate-300" />
        {/if}
      </div>
      <input
        {...combobox.input}
        class="bg-background-alt dark:text-grey w-full rounded-xl border border-slate-300 px-10 py-3 text-[14px]/[22px] font-medium text-slate-800 dark:border-slate-600"
        placeholder={placeholder || $LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_COUNTRY_PLACEHOLDER()}
        value={combobox.value}
      />
      <button
        {...combobox.trigger}
        class="absolute right-1 top-1/2 grid size-10 -translate-y-1/2 place-items-center rounded-lg"
      >
        <CaretDownBoldIcon class="text-primary size-5" />
      </button>
    </div>
  </div>

  <div
    {...combobox.content}
    class="hide-scrollbar bg-background-alt m-0 max-h-[232px] rounded-xl border border-slate-300 p-2 dark:border-slate-600"
  >
    {#each filtered as option (option)}
      <div {...combobox.getOption(option)} class="hover:bg-background flex items-center rounded-lg p-2">
        <Icon
          class="mr-2 size-5"
          icon={`circle-flags:${countries.find((c) => c.name === option)?.code.toLowerCase()}`}
        />
        <div class="dark:text-grey grow text-left text-[13px]/[24px] font-medium text-slate-800">{option}</div>
        {#if combobox.isSelected(option)}
          <CheckBoldIcon class="text-primary size-5" />
        {/if}
      </div>
    {:else}
      <span class="text-[13px]/[24px] font-medium text-slate-500 p-2">
        {$LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_COUNTRY_NO_MATCH()}
      </span>
    {/each}
  </div>
</div>
