<script lang="ts">
    import { dispatch } from '../dispatcher';
    import {state} from '$src/stores';
</script>   

<div class=" w-full rounded-t-xl bg-white">
    <h3 class="m-4 font-medium">Sort Connections by</h3>
    <div>
        <ul>
            <li class="m-4 font-medium">Name: A to Z</li>
            <li class="m-4 font-medium">Date: Newest to Oldest</li>
            <li class="m-4 font-medium">Issued: Newest to Oldest</li>
        </ul>
        <button class="bg-primary text-slate-200 font-medium rounded-xl m-4 w-5/6 h-12">Continue</button>
    </div>
 </div>