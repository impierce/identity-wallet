<script lang="ts">
  import {
    AlertDialog,
    AlertDialogTrigger,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogCancel,
    AlertDialogAction,
    Checkbox
  } from '@impierce/ui-components';

  export let rootOpen: boolean = false;
  export let title: string = 'title';
  export let options: string[] = [];
</script>

<AlertDialog bind:open={rootOpen}>
  <!-- <AlertDialogTrigger>
    <button>Open</button>
  </AlertDialogTrigger> -->
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{title}</AlertDialogTitle>
      <AlertDialogDescription>
        <div class="flex flex-col space-y-4">
          {#each options as option, i}
            <div class="flex items-center space-x-2">
              <Checkbox id={`${i}-${option}`} />
              <label
                for={`${i}-${option}`}
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
              >
                {option}
              </label>
            </div>
          {/each}
        </div>
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>Cancel</AlertDialogCancel>
      <AlertDialogAction>Share</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
