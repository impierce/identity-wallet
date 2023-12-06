<script lang="ts">
  import { dispatch } from '../dispatcher';
  import { state } from '$src/stores';
  import Slider from '~icons/ph/sliders-horizontal';
</script>

<div class="sort_wrapper">
  <button class=" flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark">
    <Slider class="h-5 w-5 fill-dark dark:fill-white"></Slider>
  </button>
</div>
