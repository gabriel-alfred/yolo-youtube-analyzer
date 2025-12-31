<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import Button from "$lib/components/ui/Button.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import ErrorMessage from "$lib/components/ui/ErrorMessage.svelte";
  import ConfirmationDialog from "$lib/components/ui/ConfirmationDialog.svelte";
  import { loadAppConfig, saveAppConfig, type AppConfig } from "$lib/config";

  // State
  let showCleanupDialog = $state(false);
  let isInitialized = $state(false); // Prevent auto-save during initial load

  // Settings
  let language = $state("es");
  let processingDevice = $state<"cpu" | "cuda" | "mps">("cpu");
  let compressResults = $state(true);
  let maxStorageSize = $state(50);
  let modelsPath = $state("");

  // Storage tracking
  let resultsSize = $state(0);
  let modelsSize = $state(0);
  let isLoadingStorage = $state(true);

  const languages = [
    { id: "es", name: "Español", flag: "🇪🇸" },
    { id: "en", name: "English", flag: "🇺🇸" },
    { id: "eu", name: "Euskera", flag: "🟥🟩⬜" },
  ];

  // Auto-save settings when they change
  $effect(() => {
    // Skip auto-save until initial config is loaded
    if (!isInitialized) return;

    const config: AppConfig = {
      processingDevice,
      language,
      compressResults,
      maxStorageSize,
    };

    saveAppConfig(config);

    // Dispatch storage event for other tabs
    window.dispatchEvent(
      new StorageEvent("storage", {
        key: "yolo_app_config",
        newValue: JSON.stringify(config),
      }),
    );
  });

  onMount(async () => {
    // Load saved settings independently of backend status
    try {
      const config = loadAppConfig();
      language = config.language;
      processingDevice = config.processingDevice;
      compressResults = config.compressResults;
      maxStorageSize = config.maxStorageSize;
    } catch (e) {
      console.error("Error loading app config:", e);
    }

    // Mark as initialized to enable auto-save
    isInitialized = true;

    // Load backend data
    try {
      modelsPath = await invoke("get_current_models_dir");
      await loadStorageUsage();
    } catch (err) {
      console.error("Error loading models path:", err);
    }
  });

  async function loadStorageUsage() {
    try {
      isLoadingStorage = true;
      const usage = await invoke<{ results_size: number; models_size: number }>(
        "get_storage_usage",
      );
      resultsSize = usage.results_size;
      modelsSize = usage.models_size;
    } catch (err) {
      console.error("Error loading storage usage:", err);
    } finally {
      isLoadingStorage = false;
    }
  }

  async function changeModelsFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Seleccionar carpeta de modelos",
      });

      if (selected && typeof selected === "string") {
        await invoke("set_models_dir", { path: selected });
        modelsPath = selected;
      }
    } catch (err) {
      console.error("Error al cambiar la carpeta:", err);
    }
  }

  function handleSave() {
    // This function is no longer needed but keeping it in case it's called
  }

  function handleCleanup() {
    showCleanupDialog = true;
  }

  async function confirmCleanup() {
    try {
      await invoke("cleanup_storage");
      showCleanupDialog = false;
      // Reload storage usage
      await loadStorageUsage();
    } catch (err) {
      console.error("Error cleaning storage:", err);
      alert("Error al limpiar el almacenamiento: " + err);
    }
  }
</script>

<!-- Header -->
<div class="text-center mb-12">
  <h1
    class="text-5xl font-bold bg-gradient-to-r from-red-400 via-orange-400 to-red-500 bg-clip-text text-transparent mb-4"
  >
    Configuración
  </h1>
  <p class="text-slate-300 text-lg">
    Personaliza el comportamiento de la aplicación
  </p>
</div>

<div class="max-w-4xl mx-auto space-y-6">
  <!-- Language -->
  <Card variant="default" padding="lg">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg
          class="w-6 h-6 text-red-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129"
          />
        </svg>
        Idioma
      </h2>
    {/snippet}

    <div class="grid grid-cols-3 gap-4">
      <!-- Español -->
      <button
        onclick={() => {
          language = "es";
        }}
        class="p-4 rounded-lg border transition-all text-center {language ===
        'es'
          ? 'bg-red-500/20 border-red-500/50 shadow-lg shadow-red-500/20'
          : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'}"
      >
        <div class="w-full flex items-center justify-center mb-3">
          <div
            class="w-24 h-12 rounded overflow-hidden border border-red-500/30"
          >
            <div class="h-1/4 bg-red-600"></div>
            <div class="h-2/4 bg-yellow-400"></div>
            <div class="h-1/4 bg-red-600"></div>
          </div>
        </div>
        <p class="text-sm font-medium text-red-100">Español</p>
      </button>

      <!-- English -->
      <button
        onclick={() => {
          language = "en";
        }}
        class="p-4 rounded-lg border transition-all text-center {language ===
        'en'
          ? 'bg-red-500/20 border-red-500/50 shadow-lg shadow-red-500/20'
          : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'}"
      >
        <div class="w-full flex items-center justify-center mb-3">
          <div
            class="w-24 h-12 relative bg-blue-700 rounded overflow-hidden border border-red-500/30"
          >
            <svg viewBox="0 0 60 30" class="w-full h-full">
              <!-- Diagonales blancas finas -->
              <path
                d="M0,0 L60,30 M60,0 L0,30"
                stroke="white"
                stroke-width="6"
              />
              <!-- Diagonales rojas más finas -->
              <path
                d="M0,0 L60,30 M60,0 L0,30"
                stroke="#C8102E"
                stroke-width="4"
              />
              <!-- Cruz blanca -->
              <path
                d="M0,15 L60,15 M30,0 L30,30"
                stroke="white"
                stroke-width="10"
              />
              <!-- Cruz roja -->
              <path
                d="M0,15 L60,15 M30,0 L30,30"
                stroke="#C8102E"
                stroke-width="6"
              />
            </svg>
          </div>
        </div>
        <p class="text-sm font-medium text-red-100">English</p>
      </button>

      <!-- Euskera -->
      <button
        onclick={() => {
          language = "eu";
        }}
        class="p-4 rounded-lg border transition-all text-center {language ===
        'eu'
          ? 'bg-red-500/20 border-red-500/50 shadow-lg shadow-red-500/20'
          : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'}"
      >
        <div class="w-full flex items-center justify-center mb-3">
          <div
            class="w-24 h-12 relative bg-red-600 rounded overflow-hidden border border-red-500/30"
          >
            <svg viewBox="0 0 100 50" class="w-full h-full">
              <!-- Fondo rojo completo -->
              <rect x="0" y="0" width="100" height="50" fill="#D52B1E" />

              <!-- Diagonales verdes -->
              <path d="M0,0 L50,25" stroke="#009B48" stroke-width="8" />
              <path d="M100,0 L50,25" stroke="#009B48" stroke-width="8" />
              <path d="M0,50 L50,25" stroke="#009B48" stroke-width="8" />
              <path d="M100,50 L50,25" stroke="#009B48" stroke-width="8" />

              <!-- Cruz blanca central -->
              <rect x="0" y="23" width="100" height="4" fill="white" />
              <rect x="48" y="0" width="4" height="50" fill="white" />
            </svg>
          </div>
        </div>
        <p class="text-sm font-medium text-red-100">Euskera</p>
      </button>
    </div>
  </Card>

  <!-- Model and Processing -->
  <Card variant="default" padding="lg">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg
          class="w-6 h-6 text-red-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"
          />
        </svg>
        Procesamiento
      </h2>
    {/snippet}

    <div class="space-y-6">
      <!-- Device Selection -->
      <div>
        <div class="block text-sm font-medium text-red-200 mb-3">
          Dispositivo de procesamiento
        </div>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <button
            onclick={() => {
              processingDevice = "cpu";
            }}
            class="p-6 rounded-lg border transition-all text-center {processingDevice ===
            'cpu'
              ? 'bg-red-500/20 border-red-500/50'
              : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'}"
          >
            <svg
              class="w-12 h-12 mx-auto mb-3 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"
              />
            </svg>
            <p class="text-lg font-bold text-red-100 mb-1">CPU</p>
            <p class="text-xs text-red-300/70">Mayor compatibilidad</p>
          </button>

          <button
            onclick={() => {
              processingDevice = "cuda";
            }}
            class="p-6 rounded-lg border transition-all text-center {processingDevice ===
            'cuda'
              ? 'bg-red-500/20 border-red-500/50'
              : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'}"
          >
            <svg
              class="w-12 h-12 mx-auto mb-3 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
            <p class="text-lg font-bold text-red-100 mb-1">GPU (CUDA)</p>
            <p class="text-xs text-red-300/70">Máximo rendimiento</p>
          </button>

          <button
            onclick={() => {
              processingDevice = "mps";
            }}
            class="p-6 rounded-lg border transition-all text-center {processingDevice ===
            'mps'
              ? 'bg-red-500/20 border-red-500/50'
              : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'}"
          >
            <svg
              class="w-12 h-12 mx-auto mb-3 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
              />
            </svg>
            <p class="text-lg font-bold text-red-100 mb-1">Mac (MPS)</p>
            <p class="text-xs text-red-300/70">Apple Silicon</p>
          </button>
        </div>
      </div>
    </div>
  </Card>

  <!-- Storage -->
  <Card variant="default" padding="lg">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg
          class="w-6 h-6 text-red-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"
          />
        </svg>
        Almacenamiento
      </h2>
    {/snippet}

    <div class="space-y-6">
      <!-- Models Path -->
      <div>
        <div class="text-sm font-medium text-red-200 mb-2">
          Ubicación de Modelos
        </div>
        <div class="flex gap-3">
          <div
            class="flex-1 p-3 bg-slate-900/60 border border-red-500/20 rounded-lg text-sm text-red-100/80 font-mono truncate"
          >
            {modelsPath || "Cargando..."}
          </div>
          <Button variant="secondary" onclick={changeModelsFolder}>
            {#snippet icon()}
              <svg
                class="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                />
              </svg>
            {/snippet}
            Cambiar
          </Button>
        </div>
        <p class="text-xs text-red-300/60 mt-1">
          Carpeta donde se guardan los modelos descargados
        </p>
      </div>

      <!-- Compress Results -->
      <label
        for="compressResults"
        class="flex items-center gap-3 cursor-pointer group p-3 bg-slate-900/40 border border-red-500/20 rounded-lg hover:border-red-500/40 transition-all"
      >
        <input
          id="compressResults"
          type="checkbox"
          bind:checked={compressResults}
          class="w-5 h-5 rounded border-red-500/30 text-red-500 focus:ring-red-500/50 bg-slate-800 cursor-pointer"
        />
        <div>
          <span
            class="text-sm font-medium text-red-100 group-hover:text-red-300 transition-colors"
          >
            Comprimir resultados
          </span>
          <p class="text-xs text-red-300/60">
            Reduce el tamaño de los archivos de salida
          </p>
        </div>
      </label>

      <!-- Storage Limit -->
      <div>
        <div class="flex items-center justify-between mb-2">
          <div class="text-sm font-medium text-red-200">
            Límite de almacenamiento
          </div>
          <span class="text-lg font-bold text-red-100">{maxStorageSize} GB</span
          >
        </div>
        <input
          type="range"
          min="10"
          max="500"
          step="10"
          bind:value={maxStorageSize}
          oninput={() => {}}
          class="w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-red-500"
        />
        <div class="flex justify-between text-xs text-red-100/60 mt-1">
          <span>10 GB</span>
          <span>250 GB</span>
          <span>500 GB</span>
        </div>
      </div>

      <!-- Current Usage -->
      <div class="p-4 bg-slate-900/40 border border-red-500/20 rounded-lg">
        <h3 class="text-sm font-semibold text-red-100 mb-3">
          Uso actual de almacenamiento
        </h3>
        {#if isLoadingStorage}
          <div class="text-center text-red-300/60 py-4">Calculando...</div>
        {:else}
          <div class="space-y-3">
            <div>
              <div class="flex items-center justify-between text-sm mb-1">
                <span class="text-red-200">Resultados de análisis</span>
                <span class="font-semibold text-red-100"
                  >{(resultsSize / 1024 ** 3).toFixed(2)} GB</span
                >
              </div>
              <div
                class="w-full h-2 bg-slate-900/60 rounded-full overflow-hidden"
              >
                <div
                  class="h-full bg-gradient-to-r from-red-500 to-orange-500"
                  style="width: {Math.min(
                    100,
                    (resultsSize / 1024 ** 3 / maxStorageSize) * 100,
                  )}%"
                ></div>
              </div>
            </div>
            <div>
              <div class="flex items-center justify-between text-sm mb-1">
                <span class="text-red-200">Modelos descargados</span>
                <span class="font-semibold text-red-100"
                  >{(modelsSize / 1024 ** 3).toFixed(2)} GB</span
                >
              </div>
              <div
                class="w-full h-2 bg-slate-900/60 rounded-full overflow-hidden"
              >
                <div
                  class="h-full bg-gradient-to-r from-orange-500 to-red-500"
                  style="width: {Math.min(
                    100,
                    (modelsSize / 1024 ** 3 / maxStorageSize) * 100,
                  )}%"
                ></div>
              </div>
            </div>
            <div class="pt-2 border-t border-red-500/20">
              <div class="flex items-center justify-between text-sm">
                <span class="text-red-200 font-medium">Total</span>
                <span class="font-bold text-red-100"
                  >{((resultsSize + modelsSize) / 1024 ** 3).toFixed(2)} GB / {maxStorageSize}
                  GB</span
                >
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Cleanup Button -->
      <Button variant="danger" fullWidth onclick={handleCleanup}>
        {#snippet icon()}
          <svg
            class="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
            />
          </svg>
        {/snippet}
        Limpiar almacenamiento
      </Button>
    </div>
  </Card>
</div>

<!-- Cleanup Confirmation Dialog -->
<ConfirmationDialog
  bind:open={showCleanupDialog}
  variant="danger"
  title="¿Limpiar almacenamiento?"
  message="Esta acción eliminará TODOS los resultados de análisis y modelos descargados. Esta acción no se puede deshacer."
  confirmText="Eliminar todo"
  cancelText="Cancelar"
  onConfirm={confirmCleanup}
/>

<style>
  :global(body) {
    background: linear-gradient(
      to bottom right,
      rgb(2 6 23),
      rgb(15 23 42),
      rgb(2 6 23)
    );
    min-height: 100vh;
  }
</style>
