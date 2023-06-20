<script lang="ts">
  import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger
  } from '@impierce/ui-components';

  interface Credential {
    id: string;
    issuanceDate: string;
    issuer: any;
    credentialSubject: any;
  }

  export let credential: Credential;
</script>

<div class="break-all pt-4">
  <div class="flex flex-col space-y-1 pb-4">
    <!-- <div class="flex items-center justify-between">
      <p class="font-medium text-slate-300">id</p>  
    </div> -->

    <!-- <div class="flex flex-col items-start rounded bg-slate-100 p-2">
      <p class="pb-2 font-medium text-slate-500">Issuer</p>
      <div class="space-y-2">
        {#each Object.entries(credential.issuer) as [key, value]}
          <div class="flex items-center justify-between">
            <p class="font-medium text-slate-300">{key}</p>
            <p class="ml-4 rounded-sm bg-slate-200 px-3 py-1 font-medium text-slate-500">{value}</p>
          </div>
        {/each}
      </div>
    </div> -->
  </div>

  {#each Object.entries(credential.credentialSubject) as [key, value]}
    <div class="flex justify-between">
      <p class="font-medium text-slate-300">{key}</p>
      <p class="w-1/2 text-right font-medium text-slate-500">{value}</p>
    </div>
  {/each}

  <Accordion type="single" collapsible>
    <AccordionItem value="item-1">
      <AccordionTrigger>Metadata</AccordionTrigger>
      <AccordionContent>
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <p class="font-medium text-slate-300">ID</p>
            <p class="rounded-sm bg-slate-200 px-3 py-1 font-medium text-slate-500">
              {credential.id}
            </p>
          </div>

          <div class="flex items-start justify-between">
            <p class="font-medium text-slate-300">type</p>
            <div class="space-y-1">
              {#each credential.type as type}
                <p class="rounded-sm bg-slate-200 px-3 py-1 font-medium text-slate-500">{type}</p>
              {/each}
            </div>
            <!-- <p class="font-medium text-slate-500">{JSON.stringify(credential.type, null, 2)}</p> -->
          </div>

          <div class="flex justify-between">
            <p class="font-medium text-slate-300">type</p>
            <div class="grid grid-flow-row-dense">
              {#each credential.type as type}
                <p class="rounded-sm bg-slate-200 px-3 py-1 font-medium text-slate-500">{type}</p>
              {/each}
            </div>
            <!-- <p class="font-medium text-slate-500">{JSON.stringify(credential.type, null, 2)}</p> -->
          </div>

          <div class="flex items-center justify-between">
            <p class="font-medium text-slate-300">issuanceDate</p>
            <p class="rounded-sm bg-slate-200 px-3 py-1 font-medium text-slate-500">
              {credential.issuanceDate}
            </p>
          </div>

          <div>Issuer</div>
          {#if typeof credential.issuer === 'string'}
            <div class="flex items-center justify-between">
              <p class="font-medium text-slate-300">issuer</p>
              <p class="ml-4 rounded-sm bg-slate-200 px-3 py-1 font-medium text-slate-500">
                {credential.issuer}
              </p>
            </div>
          {:else}
            {#each Object.entries(credential.issuer) as [key, value]}
              <div class="flex items-center justify-between">
                <p class="font-medium text-slate-300">{key}</p>
                <p class="ml-4 rounded-sm bg-slate-200 px-3 py-1 font-medium text-slate-500">
                  {value}
                </p>
              </div>
            {/each}
          {/if}
        </div>
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</div>
