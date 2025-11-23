<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import { 
    COCO_CLASSES, 
    DEFAULT_MODELS, 
    getDownloadedModels, 
    mergeModelsWithDownloads, 
    type Model 
  } from '$lib/models';

  // Colores predefinidos
  const PRESET_COLORS = [
    '#ef4444', '#f97316', '#f59e0b', '#eab308', '#84cc16', '#22c55e',
    '#10b981', '#14b8a6', '#06b6d4', '#0ea5e9', '#3b82f6', '#6366f1',
    '#8b5cf6', '#a855f7', '#d946ef', '#ec4899', '#f43f5e'
  ];

  let models = $state<Model[]>([]);
  let availableModels = $derived(models.filter(m => m.downloaded));

  let youtubeUrl = $state('');
  let selectedModel = $state('');
  let modelDropdownOpen = $state(false);
  let frames = $state(1);
  let quality = $state(75);
  let minPrecision = $state(50);
  let analyzing = $state(false);
  let progress = $state(0);
  let previewUrl = $state('');
  
  let selectedClasses = $state(
    COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
  );
  
  let classColors = $state(
    COCO_CLASSES.reduce((acc, cls, idx) => ({ ...acc, [cls]: PRESET_COLORS[idx % PRESET_COLORS.length] }), {})
  );

  onMount(async () => {
    const downloadedFiles = await getDownloadedModels();
    models = mergeModelsWithDownloads(DEFAULT_MODELS, downloadedFiles);
    // Select first available model if any
    if (availableModels.length > 0 && !selectedModel) {
      selectedModel = availableModels[0].name;
    }
  });

  function extractYoutubeId(url: string) {
    const regExp = /^.*((youtu.be\/)|(v\/)|(\/u\/\w\/)|(embed\/)|(watch\?))\??v?=?([^#&?]*).*/;
    const match = url.match(regExp);
    return (match && match[7].length === 11) ? match[7] : null;
  }

  function handleAnalyze() {
    analyzing = true;
    progress = 0;
    
    const interval = setInterval(() => {
      progress += 2;
      if (progress >= 100) {
        clearInterval(interval);
        progress = 100;
      }
    }, 100);
  }

  function toggleClass(className: string) {
    selectedClasses[className] = !selectedClasses[className];
  }

  function toggleAllClasses() {
    const allSelected = Object.values(selectedClasses).every(v => v);
    COCO_CLASSES.forEach(cls => {
      selectedClasses[cls] = !allSelected;
    });
  }

  // Watch for URL changes to update preview
  $effect(() => {
    const videoId = extractYoutubeId(youtubeUrl);
    if (videoId) {
      previewUrl = `https://img.youtube.com/vi/${videoId}/maxresdefault.jpg`;
    } else {
      previewUrl = '';
    }
  });
</script>

<!-- Header -->
<div class="text-center mb-12">
  <h1 class="text-5xl font-bold bg-gradient-to-r from-red-400 via-orange-400 to-red-500 bg-clip-text text-transparent mb-4">
    Análisis de Video
  </h1>
  <p class="text-slate-300 text-lg">
    Configura y ejecuta el análisis YOLO en videos de YouTube
  </p>
</div>

<!-- Single Column Layout -->
<div class="max-w-5xl mx-auto space-y-6">
  <!-- YouTube URL Input -->
  <Card variant="default" padding="lg">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
        </svg>
        URL del Video
      </h2>
    {/snippet}
    
    <div class="flex gap-3 items-end">
      <div class="flex-1">
        <Input
          type="url"
          bind:value={youtubeUrl}
          placeholder="https://www.youtube.com/watch?v=..."
        >
          {#snippet icon()}
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
          {/snippet}
        </Input>
      </div>
      <Button
        variant="primary"
        size="lg"
        disabled={!youtubeUrl || !selectedModel || analyzing}
        onclick={handleAnalyze}
        class="whitespace-nowrap"
      >
        {#snippet icon()}
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        {/snippet}
        {analyzing ? 'Analizando...' : 'Analizar'}
      </Button>
    </div>

    <!-- Progress Bar -->
    {#if analyzing}
      <div class="mt-6">
        <div class="flex items-center justify-between mb-2">
          <h3 class="text-sm font-semibold text-red-200">Progreso del Análisis</h3>
          <span class="text-sm text-red-300 font-medium">{progress}%</span>
        </div>
        <div class="w-full h-3 bg-slate-900/60 rounded-full overflow-hidden border border-red-500/30">
          <div
            class="h-full bg-gradient-to-r from-red-500 to-orange-500 transition-all duration-300 relative overflow-hidden"
            style="width: {progress}%"
          >
            <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent animate-shimmer"></div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Video Preview -->
    {#if previewUrl}
      <div class="mt-6 flex justify-center">
        <div class="w-full max-w-md">
          <h3 class="text-sm font-semibold text-red-200 mb-3">Vista Previa</h3>
          <div class="relative rounded-lg overflow-hidden border border-red-500/30 shadow-lg shadow-red-500/20">
            <img
              src={previewUrl}
              alt="YouTube Preview"
              class="w-full h-auto"
              onerror={(e) => {
                e.currentTarget.src = `https://img.youtube.com/vi/${extractYoutubeId(youtubeUrl)}/hqdefault.jpg`;
              }}
            />
            {#if analyzing}
              <div class="absolute inset-0 bg-gradient-to-t from-slate-900/80 to-transparent flex items-end p-3">
                <div class="flex items-center gap-2 text-red-100">
                  <div class="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
                  <span class="text-xs font-medium">Procesando...</span>
                </div>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {/if}
  </Card>

  <!-- Model and Parameters -->
  <Card variant="default" padding="lg" class="overflow-visible">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
        Configuración del Modelo
      </h2>
    {/snippet}
    
    <div class="space-y-5">
      <!-- Model Selector - Full Width -->
      <div class="relative">
        <label for="model-selector" class="block text-sm font-medium text-red-200 mb-2">
          Modelo YOLO
        </label>
        <div class="relative z-50">
          <button
            id="model-selector"
            onclick={() => modelDropdownOpen = !modelDropdownOpen}
            class="w-full px-4 py-2.5 bg-slate-900/60 border border-red-500/30 rounded-lg text-left text-red-50 hover:border-red-500/50 transition-all flex items-center justify-between focus:outline-none focus:border-red-500"
          >
            <span>{selectedModel || (availableModels.length > 0 ? 'Seleccionar modelo' : 'No hay modelos descargados')}</span>
            <svg class="w-5 h-5 text-red-400 transition-transform {modelDropdownOpen ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>
          
          {#if modelDropdownOpen}
            <div class="absolute top-full left-0 right-0 mt-2 border-2 border-red-500/50 rounded-lg shadow-2xl z-[100]" style="background-color: #1e293b;">
              {#if availableModels.length === 0}
                <div class="px-4 py-3 text-red-300 text-sm">
                  No hay modelos descargados. Ve a la pestaña Modelos para descargar uno.
                </div>
              {:else}
                {#each availableModels as model}
                  <button
                    onclick={() => {
                      selectedModel = model.name;
                      modelDropdownOpen = false;
                    }}
                    class="w-full px-4 py-3 text-left text-red-100 transition-colors flex items-center justify-between first:rounded-t-lg last:rounded-b-lg"
                    style="background-color: #1e293b;"
                    onmouseenter={(e) => e.currentTarget.style.backgroundColor = '#991b1b30'}
                    onmouseleave={(e) => e.currentTarget.style.backgroundColor = '#1e293b'}
                  >
                    <span class="font-medium text-base">{model.name}</span>
                    {#if selectedModel === model.name}
                      <svg class="w-5 h-5 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                      </svg>
                    {/if}
                  </button>
                {/each}
              {/if}
            </div>
          {/if}
        </div>
      </div>

      <!-- Parameters Grid -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <!-- Frames -->
        <Input
          type="number"
          label="Frames (cada N frames)"
          bind:value={frames}
          min="1"
          fullWidth
        />

        <!-- Quality -->
        <Input
          type="number"
          label="Calidad (%)"
          bind:value={quality}
          min="50"
          max="100"
          fullWidth
        />

        <!-- Min Precision -->
        <Input
          type="number"
          label="Precisión Mínima (%)"
          bind:value={minPrecision}
          max="100"
          fullWidth
        />
      </div>
    </div>
  </Card>

  <!-- Classes Selection -->
  <Card variant="default" padding="lg">
    {#snippet header()}
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
          <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
          </svg>
          Clases a Detectar
        </h2>
        <Button variant="ghost" size="sm" onclick={toggleAllClasses}>
          {Object.values(selectedClasses).every(v => v) ? 'Deseleccionar Todo' : 'Seleccionar Todo'}
        </Button>
      </div>
    {/snippet}

    <div class="max-h-96 overflow-y-auto pr-2 space-y-2 custom-scrollbar">
      {#each COCO_CLASSES as className}
        <div class="flex items-center gap-3 p-3 bg-slate-900/40 border border-red-500/20 rounded-lg hover:border-red-500/40 transition-all">
          <!-- Checkbox -->
          <input
            type="checkbox"
            checked={selectedClasses[className]}
            onchange={() => toggleClass(className)}
            class="w-5 h-5 rounded border-red-500/30 text-red-500 focus:ring-red-500/50 bg-slate-800 cursor-pointer flex-shrink-0"
          />
          
          <!-- Class Name -->
          <span class="flex-1 text-sm font-medium {selectedClasses[className] ? 'text-red-100' : 'text-red-300/50'}">
            {className}
          </span>
          
          <!-- Color Picker -->
          <input
            type="color"
            bind:value={classColors[className]}
            disabled={!selectedClasses[className]}
            class="w-10 h-10 rounded-lg cursor-pointer border-2 border-red-500/30 hover:border-red-500/50 transition-all disabled:opacity-30 disabled:cursor-not-allowed flex-shrink-0"
          />
        </div>
      {/each}
    </div>
  </Card>

  <!-- Info Card -->
  <Card variant="gradient" padding="lg">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        Información
      </h2>
    {/snippet}
    
    <div class="grid sm:grid-cols-2 gap-4 text-sm text-red-200/80">
      <div class="flex items-start gap-2">
        <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p>El análisis procesa el video frame por frame según la configuración</p>
      </div>
      <div class="flex items-start gap-2">
        <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p>Mayor calidad mejora la detección pero aumenta el tiempo de procesamiento</p>
      </div>
      <div class="flex items-start gap-2">
        <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p>Ajusta la precisión mínima para filtrar detecciones menos confiables</p>
      </div>
      <div class="flex items-start gap-2">
        <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p>Selecciona solo las clases necesarias para optimizar el rendimiento</p>
      </div>
    </div>
  </Card>
</div>

<style>
  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
  
  .animate-shimmer {
    animation: shimmer 2s infinite;
  }

  /* Custom scrollbar */
  .custom-scrollbar::-webkit-scrollbar {
    width: 8px;
  }
  
  .custom-scrollbar::-webkit-scrollbar-track {
    background: rgba(15, 23, 42, 0.5);
    border-radius: 4px;
  }
  
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(239, 68, 68, 0.3);
    border-radius: 4px;
  }
  
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(239, 68, 68, 0.5);
  }
</style>