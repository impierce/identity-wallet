<!-- https://animation-svelte.vercel.app/magic/circular-progress-bar -->

<script lang="ts">
  import { clsx, type ClassValue } from 'clsx';
  import { twMerge } from 'tailwind-merge';

  //   import { cn } from '$lib/utils';

  export let max: number = 100;
  export let value: number = 0;
  export let displayValue: string = '';
  export let min: number = 0;
  export let gaugePrimaryColor: string = '#f00';
  // gaugeSecondaryColor is set through Tailwind class: "stroke-*"

  let className: string = '';
  export { className as class };

  let circumference = 2 * Math.PI * 45;
  let percentPx = circumference / 100;
  let currentPercent = ((value - min) / (max - min)) * 100;

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }
</script>

<div
  class={cn('relative size-40 text-2xl font-semibold', className)}
  style:--circle-size="100px"
  style:--circumference={circumference}
  style:--percent-to-px="{percentPx}px"
  style:--gap-percent="5"
  style:--offset-factor="0"
  style:--transition-length="1s"
  style:--transition-step="200ms"
  style:--delay="0s"
  style:--percent-to-deg="3.6deg"
  style="transform: translateZ(0);"
>
  <svg fill="none" class="size-full" stroke-width="2" viewBox="0 0 100 100">
    {#if currentPercent <= 90 && currentPercent >= 0}
      <circle
        cx="50"
        cy="50"
        r="45"
        stroke-width="10"
        stroke-dashoffset="0"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="opacity-100"
        style="
        stroke-dasharray: calc(var(--stroke-percent) * var(--percent-to-px)) var(--circumference);
        transform: rotate(calc(1turn - 90deg - (var(--gap-percent) * var(--percent-to-deg) * var(--offset-factor-secondary)))) scaleY(-1);
        transition: all var(--transition-length) ease var(--delay);
        transform-origin:calc(var(--circle-size) / 2) calc(var(--circle-size) / 2);
        "
        style:--stroke-percent={90 - currentPercent}
        style:--offset-factor-secondary="calc(1 - var(--offset-factor))"
      />
    {/if}
    <circle
      cx="50"
      cy="50"
      r="45"
      stroke-width="10"
      stroke-dashoffset="0"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="opacity-100"
      style="
        stroke:{gaugePrimaryColor};
        stroke-dasharray:calc(var(--stroke-percent) * var(--percent-to-px)) var(--circumference);
        transition:var(--transition-length) ease var(--delay),stroke var(--transition-length) ease var(--delay);
        transition-property: stroke-dasharray,transform;
        transform:rotate(calc(-90deg + var(--gap-percent) * var(--offset-factor) * var(--percent-to-deg)));
          transform-origin:calc(var(--circle-size) / 2) calc(var(--circle-size) / 2);
        "
      style:--stroke-percent={currentPercent}
    />
  </svg>
  <span
    data-current-value={currentPercent}
    class="animate-in fade-in absolute inset-0 m-auto size-fit delay-[var(--delay)] duration-[var(--transition-length)] ease-linear"
  >
    <!-- {currentPercent.toFixed(0)} -->
    <div class="text-center">
      {displayValue}
      <!-- <p class="text-xs font-medium text-slate-500">seconds left</p> -->
    </div>
  </span>
</div>
