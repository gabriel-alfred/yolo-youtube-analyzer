<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import Button from '$lib/components/ui/Button.svelte';
  
  let scrolled = $state(false);
  let mobileMenuOpen = $state(false);
  
  const navigateToAnalysis = () => {
    goto('/analysis');
  };
  
  const navigateToHome = () => {
    goto('/');
  };
  
  const navigateToModels = () => {
    goto('/models');
  };
  
  const navigateToResults = () => {
    goto('/results');
  };
  
  const navigateToMonitoring = () => {
    goto('/monitoring');
  };
  
  const handleKeyPress = (e: KeyboardEvent, callback: () => void) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      callback();
    }
  };
  
  onMount(() => {
    const handleScroll = () => {
      scrolled = window.scrollY > 10;
    };
    
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  });
</script>

<header 
  class="sticky top-0 z-50 transition-all duration-500 ease-in-out bg-slate-950/50 backdrop-blur-sm border-b border-red-500/20"
  class:scrolled
>
  <div class="container mx-auto px-3 sm:px-4 py-3 sm:py-4 max-w-7xl">
    <div class="flex items-center justify-between">
      <!-- Logo/Brand -->
      <button 
        class="flex items-center space-x-2 sm:space-x-3 group cursor-pointer bg-transparent border-none p-0"
        onclick={navigateToHome}
        onkeydown={(e) => handleKeyPress(e, navigateToHome)}
        aria-label="Ir a la página de inicio"
      >
        <div class="relative">
          <div class="absolute inset-0 bg-red-500 blur-xl opacity-50 group-hover:opacity-75 transition-opacity"></div>
          <div class="relative w-8 h-8 sm:w-10 sm:h-10 bg-gradient-to-br from-red-500 to-orange-500 rounded-lg flex items-center justify-center shadow-lg shadow-red-500/50 group-hover:shadow-red-500/75 transition-all group-hover:scale-110">
            <svg class="w-5 h-5 sm:w-6 sm:h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
          </div>
        </div>
        
        <div>
          <h1 class="text-lg sm:text-xl md:text-2xl font-bold bg-gradient-to-r from-red-400 via-orange-400 to-red-500 bg-clip-text text-transparent group-hover:from-orange-400 group-hover:to-red-400 transition-all">
            YOLO Analyzer
          </h1>
          <p class="text-[10px] sm:text-xs text-red-400/60 font-medium tracking-wider hidden xs:block">YOUTUBE VIDEO DETECTION</p>
        </div>
      </button>

      <!-- Desktop Navigation -->
      <nav class="hidden lg:flex items-center space-x-1">
        {#each [
          { label: 'Análisis', icon: 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z' },
          { label: 'Modelos', icon: 'M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z' },
          { label: 'Resultados', icon: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' },
          { label: 'Monitoreo', icon: 'M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z' },
          { label: 'Config', icon: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z' }
        ] as item}
          <button 
            class="px-3 xl:px-4 py-2 text-sm font-medium text-red-100/80 hover:text-red-100 hover:bg-red-500/10 rounded-lg transition-all relative group flex items-center gap-2"
            onclick={() => {
              if (item.label === 'Análisis') navigateToAnalysis();
              else if (item.label === 'Modelos') navigateToModels();
              else if (item.label === 'Resultados') navigateToResults();
              else if (item.label === 'Monitoreo') navigateToMonitoring();
            }}
            aria-label={`Ir a ${item.label}`}
          >
            <svg class="w-4 h-4 opacity-70 group-hover:opacity-100 transition-opacity" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="{item.icon}" />
            </svg>
            <span class="relative z-10">{item.label}</span>
            <div class="absolute inset-0 bg-gradient-to-r from-red-500/0 via-orange-500/5 to-red-500/0 opacity-0 group-hover:opacity-100 rounded-lg transition-opacity"></div>
          </button>
        {/each}
      </nav>

      <!-- Action Buttons -->
      <div class="flex items-center space-x-2 sm:space-x-3">
        <!-- Model Status - Hidden on small screens -->
        <div class="hidden xl:flex items-center space-x-2 px-3 py-1.5 bg-green-500/10 border border-green-500/20 rounded-lg">
          <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse shadow-lg shadow-green-400/50"></div>
          <span class="text-xs font-medium text-green-300">YOLOv11 Ready</span>
        </div>

        <!-- CTA Button using Button Component -->
        <Button 
          size="sm" 
          class="text-xs sm:text-sm px-3 sm:px-4 md:px-5 py-1.5 sm:py-2"
          onclick={navigateToAnalysis}
        >
          {#snippet icon()}
            <svg class="w-3 h-3 sm:w-4 sm:h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
            </svg>
          {/snippet}
          <span class="hidden xs:inline">Nuevo Análisis</span>
          <span class="xs:hidden">Analizar</span>
        </Button>

        <!-- Mobile Menu Button -->
        <button 
          class="lg:hidden p-2 text-red-300 hover:text-red-100 hover:bg-red-500/10 rounded-lg transition-all"
          aria-label="Abrir menú de navegación"
          onclick={() => mobileMenuOpen = !mobileMenuOpen}
        >
          <svg class="w-5 h-5 sm:w-6 sm:h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            {#if mobileMenuOpen}
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            {:else}
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            {/if}
          </svg>
        </button>
      </div>
    </div>

    <!-- Mobile Menu -->
    {#if mobileMenuOpen}
      <nav class="lg:hidden mt-4 pt-4 border-t border-red-500/20 animate-in slide-in-from-top-2">
        <div class="flex flex-col space-y-1">
          <!-- Model Status in Mobile Menu -->
          <div class="xl:hidden flex items-center justify-center space-x-2 px-4 py-3 bg-green-500/10 border border-green-500/20 rounded-lg mt-2">
            <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse shadow-lg shadow-green-400/50"></div>
            <span class="text-xs font-medium text-green-300">YOLOv11 Ready</span>
          </div>
          
          {#each [
            { label: 'Análisis', icon: 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z' },
            { label: 'Modelos', icon: 'M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z' },
            { label: 'Resultados', icon: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' },
            { label: 'Monitoreo', icon: 'M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z' },
            { label: 'Configuración', icon: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z' }
          ] as item}
            <button 
              class="w-full px-4 py-3 text-sm font-medium text-red-100/80 hover:text-red-100 hover:bg-red-500/10 rounded-lg transition-all flex items-center gap-3 text-left"
              onclick={() => {
                if (item.label === 'Análisis') navigateToAnalysis();
                else if (item.label === 'Modelos') navigateToModels();
                else if (item.label === 'Resultados') navigateToResults();
                else if (item.label === 'Monitoreo') navigateToMonitoring();
                mobileMenuOpen = false;
              }}
              aria-label={`Ir a ${item.label}`}
            >
              <svg class="w-5 h-5 opacity-70" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="{item.icon}" />
              </svg>
              <span>{item.label}</span>
            </button>
          {/each}
          
        </div>
      </nav>
    {/if}
  </div>
  
  <!-- Subtle Glow Effect -->
  <div class="absolute inset-0 bg-gradient-to-b from-red-500/5 to-transparent pointer-events-none"></div>
</header>

<style>
  header {
    transition: all 0.3s ease-in-out;
    position: relative;
  }
  
  header.scrolled {
    background-color: rgba(2, 6, 23, 0.7);
    border-bottom-color: rgba(239, 68, 68, 0.3);
  }
</style>