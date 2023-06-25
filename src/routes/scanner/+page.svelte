<script lang="ts">
  import Scanner from '$lib/Scanner.svelte';

  let selected = {
    label: 'Scanner',
    component: Scanner
    // icon: 'i-ph-scan'
  };

  let isMobile = true;

  //   function insecureRenderHtml(html) {
  //     messages.update((r) => [
  //       {
  //         html:
  //           `<pre><strong class="text-accent dark:text-darkAccent">[${new Date().toLocaleTimeString()}]:</strong> ` +
  //           html +
  //           "</pre>",
  //       },
  //       ...r,
  //     ]);
  //   }

  function onMessage(value) {
    console.log(value);
  }

  const html = document.querySelector("body");
  html.classList.add("transparent");
</script>

<main
  class="transition-colors-250 grid flex-1 grid-rows-[2fr_auto] bg-orange-300 transition-transform"
  class:transparent={isMobile}
>
  <div class="flex flex-col items-center p-6">
  <!-- <div class="grid grid-rows-[auto_1fr] overflow-hidden px-5"> -->
    <h1 class="text-3xl font-medium p-6">Scanner</h1>
    <div class="overflow-y-auto">
      <div class="mr-2">
        <svelte:component this={selected.component} {onMessage} />
      </div>
    </div>
  </div>
</main>

<style>
  .transparent {
    background-color: transparent;
  }
  
  /* :root {
		background-color: transparent;
	} */
</style>
