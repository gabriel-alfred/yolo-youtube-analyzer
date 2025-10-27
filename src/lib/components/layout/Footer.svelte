<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  
  let currentYear = new Date().getFullYear();

  async function openExternal(url: string) {
    try {
      await invoke('plugin:opener|open_url', { url: url });
    } catch (error) {
      console.error('Error opening URL:', error);
    }
  }
</script>

<footer class="relative z-10 mt-auto border-t border-red-500/20 bg-slate-950/50 backdrop-blur-sm">
  <div class="container mx-auto px-3 sm:px-4 py-4 sm:py-6 max-w-7xl">
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4 items-center">
      <!-- Left: Copyright -->
      <p class="text-xs text-red-100/60 text-center sm:text-left order-2 sm:order-1">
        © {currentYear} YOLO Analyzer · Powered by YOLOv11
      </p>
      
      <!-- Center: Links -->
      <nav class="flex flex-wrap justify-center gap-3 sm:gap-4 order-1 sm:order-2">
        {#each ['Docs', 'Config', 'Contacto'] as link}
          <button 
            on:click={() => openExternal('#')}
            class="text-xs text-red-100/60 hover:text-red-300 transition-colors cursor-pointer bg-transparent border-none"
          >
            {link}
          </button>
        {/each}
      </nav>

      <!-- Right: Social Links -->
      <div class="flex items-center justify-center sm:justify-end space-x-2 order-3">
        {#each [
          { icon: 'M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z', label: 'Twitter', url: 'https://twitter.com' },
          { icon: 'M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 00-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0020 4.77 5.07 5.07 0 0019.91 1S18.73.65 16 2.48a13.38 13.38 0 00-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 005 4.77a5.44 5.44 0 00-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 009 18.13V22', label: 'GitHub', url: 'https://github.com/gabriel-alfred/yolo-youtube-analyzer' }
        ] as social}
          <button 
            on:click={() => openExternal(social.url)}
            class="p-1.5 text-red-300/60 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition-all group cursor-pointer bg-transparent border-none"
            aria-label={social.label}
          >
            <svg class="w-4 h-4 group-hover:scale-110 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="{social.icon}" />
            </svg>
          </button>
        {/each}
      </div>
    </div>
  </div>

  <!-- Subtle Glow Effect -->
  <div class="absolute inset-0 bg-gradient-to-t from-red-500/5 to-transparent pointer-events-none"></div>
</footer>