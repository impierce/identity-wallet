<script lang="ts">
  import { dispatch } from '../dispatcher';

  //here I will put the import
  //probably an export..?
</script>

<!-- The HTML and TailwindCSS of the search component-->
<div class="search_bar">
  <form>
    <h3 class="m-2 text-sm font-medium text-gray-100">Search stuff here</h3>
    <svg
      class="fixed m-3 mr-5 h-5 w-5"
      fill="none"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      viewBox="0 0 24 24"
    >
      <path d="M21 21l-5.2-5.2"></path>
      <path d="M15 11c0-3.9-3.1-7-7-7s-7 3.1-7 7 3.1 7 7 7 7-3.1 7-7"></path>
    </svg>
    <input
      type="text"
      class="mb-6 ml-1 w-11/12 rounded-full bg-white px-4 py-3 pl-8 text-sm text-gray-600"
      placeholder="abc.com..."
      on:input={(e) => dispatch({ type: '[User Data] Query', payload: { target: 'Credentials', search_term: e.target.value } })}
    />
  </form>
</div>
