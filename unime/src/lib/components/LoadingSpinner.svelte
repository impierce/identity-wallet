<script lang="ts">
  import '@lottiefiles/lottie-player';

  import { twMerge } from 'tailwind-merge';
</script>

<div class={twMerge('h-8 w-8', $$props.class)}>
  <lottie-player src="/lottiefiles/Animation-1704188137306.json" autoplay loop speed={1.25} mode="normal" />
</div>
