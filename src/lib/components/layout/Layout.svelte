<script lang="ts">
  import { onMount } from 'svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import Footer from '$lib/components/layout/Footer.svelte';
  
  let mounted = $state(false);
  
  onMount(() => {
    mounted = true;
  });
  
  interface Props {
    children?: import('svelte').Snippet;
  }
  
  let { children }: Props = $props();
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-950 via-red-950 to-slate-900 relative overflow-hidden flex flex-col">
  <!-- Animated Background Effects -->
  <div class="absolute inset-0 overflow-hidden pointer-events-none">
    <!-- Grid Pattern -->
    <div class="absolute inset-0 bg-[linear-gradient(to_right,#dc262615_1px,transparent_1px),linear-gradient(to_bottom,#dc262615_1px,transparent_1px)] bg-[size:2rem_2rem] sm:bg-[size:3rem_3rem] lg:bg-[size:4rem_4rem]"></div>
    
    <!-- Glowing Orbs -->
    <div class="absolute top-10 left-5 sm:top-20 sm:left-20 w-48 h-48 sm:w-72 sm:h-72 lg:w-96 lg:h-96 bg-red-500/20 rounded-full blur-3xl animate-pulse"></div>
    <div class="absolute bottom-10 right-5 sm:bottom-20 sm:right-20 w-48 h-48 sm:w-72 sm:h-72 lg:w-96 lg:h-96 bg-orange-500/20 rounded-full blur-3xl animate-pulse" style="animation-delay: 1s;"></div>
    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-48 h-48 sm:w-72 sm:h-72 lg:w-96 lg:h-96 bg-red-500/10 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s;"></div>
  </div>

  <!-- Header -->
  <Header />
  
  <!-- Main Content -->
  <div class="relative z-10 flex-grow">
    <main class="container mx-auto px-3 sm:px-4 md:px-6 lg:px-8 py-4 sm:py-6 md:py-8 max-w-7xl">
      <div class="{mounted ? 'opacity-100' : 'opacity-0'} transition-opacity duration-1000">
        {#if children}
          {@render children()}
        {/if}
      </div>
    </main>
  </div>

  <!-- Footer -->
  <Footer />
  
  <!-- Scanline Effect -->
  <div class="pointer-events-none absolute inset-0 bg-[linear-gradient(transparent_50%,rgba(220,38,38,0.02)_50%)] sm:bg-[linear-gradient(transparent_50%,rgba(220,38,38,0.03)_50%)] bg-[size:100%_4px] animate-scan"></div>
</div>

<style>
  @keyframes scan {
    0% {
      background-position: 0 0;
    }
    100% {
      background-position: 0 100%;
    }
  }
  
  .animate-scan {
    animation: scan 8s linear infinite;
  }
</style>