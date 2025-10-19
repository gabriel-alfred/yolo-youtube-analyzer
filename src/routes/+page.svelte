<script>
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-800 text-gray-900 dark:text-gray-100 px-4">
  <div class="pt-10 text-center">
    <h1 class="text-4xl font-semibold mb-8">Welcome to Tauri + Svelte</h1>

    <div class="flex justify-center gap-6 mb-6">
      <a href="https://vite.dev" target="_blank" class="group">
        <img 
          src="/vite.svg" 
          class="h-24 p-6 transition-all duration-700 group-hover:drop-shadow-[0_0_2em_#747bff]" 
          alt="Vite Logo" 
        />
      </a>
      <a href="https://tauri.app" target="_blank" class="group">
        <img 
          src="/tauri.svg" 
          class="h-24 p-6 transition-all duration-700 group-hover:drop-shadow-[0_0_2em_#24c8db]" 
          alt="Tauri Logo" 
        />
      </a>
      <a href="https://svelte.dev" target="_blank" class="group">
        <img 
          src="/svelte.svg" 
          class="h-24 p-6 transition-all duration-700 group-hover:drop-shadow-[0_0_2em_#ff3e00]" 
          alt="SvelteKit Logo" 
        />
      </a>
    </div>

    <p class="mb-6 font-medium text-gray-700 dark:text-gray-300">
      Click on the Tauri, Vite, and SvelteKit logos to learn more.
    </p>

    <form class="flex justify-center gap-2 mb-4" onsubmit={greet}>
      <input 
        id="greet-input" 
        placeholder="Enter a name..." 
        bind:value={name}
        class="px-5 py-2.5 text-base font-medium rounded-lg border border-transparent 
               bg-white dark:bg-gray-900/60 text-gray-900 dark:text-white
               shadow-md transition-colors duration-250 outline-none
               focus:border-blue-500"
      />
      <button 
        type="submit"
        class="px-5 py-2.5 text-base font-medium rounded-lg border border-transparent 
               bg-white dark:bg-gray-900/60 text-gray-900 dark:text-white
               shadow-md transition-all duration-250 cursor-pointer
               hover:border-blue-500 active:border-blue-500 active:bg-gray-200 dark:active:bg-gray-900/40"
      >
        Greet
      </button>
    </form>

    {#if greetMsg}
      <p class="text-lg font-medium">{greetMsg}</p>
    {/if}
  </div>
</main>