<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import Button from "$lib/components/ui/Button.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import LoadingSpinner from "$lib/components/ui/LoadingSpinner.svelte";
  import ErrorMessage from "$lib/components/ui/ErrorMessage.svelte";
  import ConfirmationDialog from "$lib/components/ui/ConfirmationDialog.svelte";
  import {
    DEFAULT_MODELS,
    getDownloadedModels,
    mergeModelsWithDownloads,
    type Model,
    COCO_CLASSES,
  } from "$lib/models";

  // State
  let streamUrl = $state("");
  let selectedModel = $state("");
  let selectedQuality = $state("medium");
  let confidenceThreshold = $state(0.5);
  let selectedDevice = $state("cpu");
  let selectedClasses = $state<string[]>([
    "person",
    "car",
    "truck",
    "bus",
    "motorcycle",
    "bicycle",
  ]);

  let isStreaming = $state(false);
  let isLoading = $state(false);
  let error = $state("");
  let statusMessage = $state("");
  let showStopDialog = $state(false);
  let showConflictDialog = $state(false);
  let showPreview = $state(false);
  let liveFrameData = $state<string | null>(null);
  let currentFps = $state(0);
  let currentDetections = $state<string[]>([]);

  // Buffering states
  type FrameData = {
    data: string;
    number: number;
    timestamp: number;
  };
  let frameBuffer = $state<FrameData[]>([]);
  let animationFrameId: number | null = null;
  let lastFrameTime = 0;
  let lastRenderedFrameNumber = -1;
  let isBuffering = $state(false);

  let models = $state<Model[]>([]);
  let availableModels = $derived(models.filter((m) => m.downloaded));

  let unlistenProgress: (() => void) | null = null;
  let unlistenError: (() => void) | null = null;

  onMount(async () => {
    const downloadedFiles = await getDownloadedModels();
    models = mergeModelsWithDownloads(DEFAULT_MODELS, downloadedFiles);
    if (availableModels.length > 0 && !selectedModel) {
      selectedModel = availableModels[0].name; // Use name for consistency with backend
    }

    // Check for active session
    try {
      const activeSession = await invoke("get_active_analysis");
      if (activeSession) {
        console.log("Found active session:", activeSession);
        const { config } = activeSession as any;

        // Only restore if it's a live analysis
        if (config.mode && config.mode !== "live") {
          return;
        }

        // Restore state
        streamUrl = config.url;
        // Try to match model name or filename
        const modelMatch = models.find(
          (m) =>
            m.fileName === config.model_name || m.name === config.model_name,
        );
        if (modelMatch) selectedModel = modelMatch.name;

        confidenceThreshold = config.conf;
        selectedDevice = config.device;

        // Restore classes
        if (config.classes) {
          const activeClassesIndices = config.classes.split(",").map(Number);
          selectedClasses = []; // Reset
          activeClassesIndices.forEach((idx: number) => {
            if (idx >= 0 && idx < COCO_CLASSES.length) {
              selectedClasses.push(COCO_CLASSES[idx]);
            }
          });
        }

        isLoading = true;
        statusMessage = "Reconectando con stream...";
      }
    } catch (e) {
      console.error("Error checking active session:", e);
    }

    unlistenProgress = await listen("analysis-progress", (event: any) => {
      // Guard: If not streaming or setting up, ignore events
      if (!isStreaming && !isLoading) return;

      const payload = event.payload;

      if (payload.status === "frame" && payload.frame_data) {
        if (payload.fps) currentFps = Math.round(payload.fps);
        if (payload.detections) currentDetections = payload.detections;

        // Push to buffer instead of direct render
        frameBuffer.push({
          data: payload.frame_data,
          number: payload.frame_number || 0,
          timestamp: payload.timestamp || 0,
        });

        isLoading = false;
        isStreaming = true;

        // Ensure loop is running
        if (!animationFrameId) {
          startPlaybackLoop();
        }
      } else if (payload.status === "error") {
        error = payload.message;
        isLoading = false;
        isStreaming = false;
      } else if (payload.message) {
        statusMessage = payload.message;
      }
    });

    unlistenError = await listen("analysis-error", (event: any) => {
      console.error("Analysis error:", event.payload);
      // Ignore non-fatal ffmpeg/opencv warnings
      const msg = String(event.payload);
      if (!msg.includes("OpenH264") && !msg.includes("FFMPEG")) {
        error = msg;
      }
    });
  });

  onDestroy(() => {
    if (unlistenProgress) unlistenProgress();
    if (unlistenError) unlistenError();
    // Do NOT stop analysis here, so it persists when switching tabs
  });

  const qualities = [
    { id: "low", name: "Baja (360p)", fps: "Rápido" },
    { id: "medium", name: "Media (720p)", fps: "Equilibrado" },
    { id: "high", name: "Alta (1080p)", fps: "Detallado" },
  ];

  // Use COCO classes from lib
  const availableClasses = COCO_CLASSES;

  function startPlaybackLoop() {
    const loop = (timestamp: number) => {
      if (!isStreaming) {
        animationFrameId = requestAnimationFrame(loop);
        return;
      }

      if (frameBuffer.length === 0) {
        if (!isBuffering) {
          isBuffering = true;
        }
        animationFrameId = requestAnimationFrame(loop);
        return;
      }

      if (isBuffering && frameBuffer.length > 5) {
        isBuffering = false;
        lastFrameTime = timestamp;
        // Start immediately
        const frame = frameBuffer.shift();
        if (frame) {
          liveFrameData = `data:image/jpeg;base64,${frame.data}`;
          lastRenderedFrameNumber = frame.number;
        }
        animationFrameId = requestAnimationFrame(loop);
        return;
      }

      if (lastRenderedFrameNumber === -1) {
        const frame = frameBuffer.shift()!;
        liveFrameData = `data:image/jpeg;base64,${frame.data}`;
        lastRenderedFrameNumber = frame.number;
        lastFrameTime = timestamp;
      } else {
        const nextFrame = frameBuffer[0];
        // Use current FPS or default to 30 if 0
        const fps = currentFps > 0 ? currentFps : 30;
        const frameDelta = nextFrame.number - lastRenderedFrameNumber;

        if (frameDelta <= 0) {
          frameBuffer.shift();
        } else {
          const requiredDelay = (frameDelta / fps) * 1000;
          const timeSinceLast = timestamp - lastFrameTime;

          if (timeSinceLast >= requiredDelay) {
            const frame = frameBuffer.shift()!;
            liveFrameData = `data:image/jpeg;base64,${frame.data}`;
            lastRenderedFrameNumber = frame.number;

            // Sync logic
            if (timeSinceLast > requiredDelay + 1000) {
              lastFrameTime = timestamp;
            } else {
              lastFrameTime += requiredDelay;
            }
          }
        }
      }

      animationFrameId = requestAnimationFrame(loop);
    };
    animationFrameId = requestAnimationFrame(loop);
  }

  // Functions
  async function handleStartStreaming() {
    if (!streamUrl.trim()) {
      error = "Por favor ingresa una URL de streaming válida";
      return;
    }

    if (!selectedModel) {
      error = "Por favor selecciona un modelo";
      return;
    }

    error = "";
    isLoading = true;
    statusMessage = "Iniciando...";
    liveFrameData = null;

    try {
      // Convert class names to indices
      const classesToDetect = selectedClasses
        .map((cls) => COCO_CLASSES.indexOf(cls))
        .filter((idx) => idx !== -1)
        .join(",");

      const modelObj = models.find((m) => m.name === selectedModel);
      const modelFileName = modelObj ? modelObj.fileName : selectedModel;

      await invoke("start_live_analysis", {
        url: streamUrl,
        modelName: modelFileName,
        conf: confidenceThreshold,
        classes: classesToDetect.length > 0 ? classesToDetect : null,
        device: selectedDevice,
        quality: selectedQuality,
      });
    } catch (e) {
      console.error("Error starting stream:", e);

      const errorMsg = String(e);
      if (errorMsg.includes("Ya hay un análisis en curso")) {
        showConflictDialog = true;
      }

      error = String(e);
      isLoading = false;
      isStreaming = false;
    }
  }

  function handleStopStreaming() {
    showStopDialog = true;
  }

  async function confirmStopStreaming() {
    try {
      await invoke("stop_video_analysis");
      isStreaming = false;
      showPreview = false;
      showStopDialog = false;
      liveFrameData = null;
      statusMessage = "";

      // Clear loop
      if (animationFrameId) cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
      frameBuffer = [];
      lastRenderedFrameNumber = -1;
    } catch (e) {
      console.error("Error stopping stream:", e);
      error = String(e);
    }
  }

  function toggleClass(className: string) {
    if (selectedClasses.includes(className)) {
      selectedClasses = selectedClasses.filter((c) => c !== className);
    } else {
      selectedClasses = [...selectedClasses, className];
    }
  }

  function selectAllClasses() {
    selectedClasses = [...availableClasses];
  }

  function clearAllClasses() {
    selectedClasses = [];
  }
</script>

<!-- Header -->
<div class="text-center mb-12">
  <h1
    class="text-5xl font-bold bg-gradient-to-r from-red-400 via-orange-400 to-red-500 bg-clip-text text-transparent mb-4"
  >
    Monitoreo en Tiempo Real
  </h1>
  <p class="text-slate-300 text-lg">
    Analiza streams en vivo con detección de objetos
  </p>
</div>

<div class="max-w-7xl mx-auto space-y-6">
  <!-- Live Indicator -->
  {#if isStreaming}
    <div class="flex justify-center">
      <div class="flex items-center gap-3">
        <div
          class="flex items-center gap-2 px-4 py-2 bg-red-500/20 border border-red-500/40 rounded-lg"
        >
          <div
            class="w-3 h-3 bg-red-500 rounded-full animate-pulse shadow-lg shadow-red-500/50"
          ></div>
          <span class="text-sm font-semibold text-red-100"
            >EN VIVO {currentFps > 0 ? `(${currentFps} FPS)` : ""} | Buffer: {frameBuffer.length}</span
          >
        </div>
        <Button variant="danger" onclick={handleStopStreaming}>
          {#snippet icon()}
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z"
                clip-rule="evenodd"
              />
            </svg>
          {/snippet}
          Detener
        </Button>
      </div>
    </div>
  {/if}

  <!-- Stream URL and Start Button -->
  <Card variant="gradient" padding="md">
    <div class="flex flex-col md:flex-row gap-3">
      <div class="flex-1">
        <Input
          bind:value={streamUrl}
          type="url"
          placeholder="https://www.youtube.com/watch?v=..."
          disabled={isStreaming || isLoading}
          fullWidth
        >
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
                d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
              />
            </svg>
          {/snippet}
        </Input>
      </div>

      {#if !isStreaming}
        <Button
          variant="primary"
          onclick={handleStartStreaming}
          disabled={!streamUrl.trim() || isLoading}
          loading={isLoading}
        >
          {#snippet icon()}
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                clip-rule="evenodd"
              />
            </svg>
          {/snippet}
          {isLoading ? "Conectando..." : "Iniciar Monitoreo"}
        </Button>
      {/if}
    </div>

    {#if error}
      <div class="mt-3">
        <ErrorMessage
          variant="error"
          message={error}
          dismissible
          onDismiss={() => (error = "")}
        />
      </div>
    {/if}
  </Card>

  <!-- Main Content -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Left Column - Stream Display and Classes (2/3 width) -->
    <div class="lg:col-span-2 space-y-6">
      <!-- Streaming Display -->
      <Card variant="elevated" padding="none">
        <div
          class="aspect-video bg-slate-950 relative overflow-hidden rounded-t-xl flex items-center justify-center"
        >
          {#if liveFrameData}
            <img
              src={liveFrameData}
              alt="Live Stream"
              class="w-full h-full object-contain"
            />
          {:else if isLoading}
            <div
              class="absolute inset-0 flex flex-col items-center justify-center bg-slate-900/80 backdrop-blur-sm z-10"
            >
              <LoadingSpinner
                size="xl"
                message={statusMessage || "Procesando stream..."}
              />
            </div>
          {:else}
            <!-- No Stream -->
            <div
              class="flex flex-col items-center justify-center p-8 text-center"
            >
              <svg
                class="w-24 h-24 text-red-500/20 mb-4"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
                />
              </svg>
              <p class="text-red-100/40 text-lg font-medium">
                Sin transmisión activa
              </p>
              <p class="text-red-100/30 text-sm mt-2">
                Configura los parámetros e inicia el monitoreo
              </p>
            </div>
          {/if}
        </div>

        <!-- Stream Info Bar -->
        <div class="p-4 bg-slate-900/50 border-t border-red-500/20">
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
            <div>
              <p class="text-xs text-red-100/60 mb-1">Modelo</p>
              <p class="text-sm font-semibold text-red-100">
                {selectedModel || "-"}
              </p>
            </div>
            <div>
              <p class="text-xs text-red-100/60 mb-1">Dispositivo</p>
              <p class="text-sm font-semibold text-red-100 uppercase">
                {selectedDevice}
              </p>
            </div>
            <div>
              <p class="text-xs text-red-100/60 mb-1">Confianza</p>
              <p class="text-sm font-semibold text-red-100">
                {(confidenceThreshold * 100).toFixed(0)}%
              </p>
            </div>
            <div>
              <p class="text-xs text-red-100/60 mb-1">Detecciones</p>
              <p class="text-sm font-semibold text-red-100">
                {currentDetections.length > 0 ? currentDetections.length : "-"}
              </p>
            </div>
          </div>
        </div>
      </Card>

      <!-- Object Classes Below Stream -->
      <Card variant="glass" padding="md">
        {#snippet header()}
          <div class="flex items-center justify-between">
            <h3
              class="text-lg font-semibold text-red-100 flex items-center gap-2"
            >
              <svg
                class="w-5 h-5 text-red-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
                />
              </svg>
              Clases de Objetos
            </h3>
            <div class="flex gap-2">
              <button
                onclick={selectAllClasses}
                disabled={isStreaming}
                class="text-xs text-red-400 hover:text-red-300 transition-colors disabled:opacity-50"
              >
                Todas
              </button>
              <button
                onclick={clearAllClasses}
                disabled={isStreaming}
                class="text-xs text-red-400 hover:text-red-300 transition-colors disabled:opacity-50"
              >
                Ninguna
              </button>
            </div>
          </div>
        {/snippet}

        <div
          class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2 max-h-60 overflow-y-auto custom-scrollbar"
        >
          {#each availableClasses as className}
            <button
              onclick={() => toggleClass(className)}
              disabled={isStreaming}
              class="px-3 py-2 rounded-lg text-xs font-medium transition-all text-center {selectedClasses.includes(
                className,
              )
                ? 'bg-red-500/20 text-red-100 border border-red-500/40'
                : 'bg-slate-800/50 text-red-100/70 border border-red-500/20 hover:border-red-500/40'} disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {className}
            </button>
          {/each}
        </div>
      </Card>
    </div>

    <!-- Right Column - Configuration Panel (1/3 width) -->
    <div class="space-y-4">
      <!-- Device Selection (New) -->
      <Card variant="glass" padding="md">
        {#snippet header()}
          <h3
            class="text-base font-semibold text-red-100 flex items-center gap-2"
          >
            <svg
              class="w-4 h-4 text-red-400"
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
            Dispositivo
          </h3>
        {/snippet}

        <select
          bind:value={selectedDevice}
          disabled={isStreaming}
          class="w-full px-3 py-2.5 bg-slate-900/60 border border-red-500/30 rounded-lg text-red-100 hover:border-red-500/50 focus:border-red-500 transition-all focus:outline-none disabled:opacity-50"
        >
          <option value="cpu">CPU (Compatible)</option>
          <option value="cuda">GPU NVIDIA (CUDA)</option>
          <option value="mps">GPU Apple (MPS)</option>
        </select>
        <p class="text-xs text-red-300/60 mt-2">
          {#if selectedDevice === "cpu"}
            Más lento, pero funciona en todos los equipos.
          {:else if selectedDevice === "cuda"}
            Requiere tarjeta NVIDIA y drivers CUDA.
          {:else if selectedDevice === "mps"}
            Optimizado para Mac con Apple Silicon.
          {/if}
        </p>
      </Card>

      <!-- Model Selection -->
      <Card variant="glass" padding="md">
        {#snippet header()}
          <h3
            class="text-base font-semibold text-red-100 flex items-center gap-2"
          >
            <svg
              class="w-4 h-4 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
              />
            </svg>
            Modelo YOLO
          </h3>
        {/snippet}

        <div class="space-y-2 max-h-48 overflow-y-auto custom-scrollbar">
          {#if availableModels.length === 0}
            <div
              class="p-4 text-center text-red-300/60 text-sm bg-slate-800/30 rounded-lg border border-red-500/10"
            >
              No hay modelos descargados. Ve a la pestaña Modelos para descargar
              uno.
            </div>
          {:else}
            {#each availableModels as model}
              <button
                onclick={() => (selectedModel = model.name)}
                disabled={isStreaming}
                class="w-full p-2.5 rounded-lg border transition-all text-left {selectedModel ===
                model.name
                  ? 'bg-red-500/20 border-red-500/50 shadow-lg shadow-red-500/20'
                  : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'} disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <div class="flex items-center justify-between">
                  <div>
                    <p class="text-sm font-semibold text-red-100">
                      {model.name}
                    </p>
                    <p class="text-xs text-red-100/60">
                      {model.speed} • {model.precision}
                    </p>
                  </div>
                  {#if selectedModel === model.name}
                    <svg
                      class="w-4 h-4 text-red-400 flex-shrink-0"
                      fill="currentColor"
                      viewBox="0 0 20 20"
                    >
                      <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  {/if}
                </div>
              </button>
            {/each}
          {/if}
        </div>
      </Card>

      <!-- Quality Settings -->
      <Card variant="glass" padding="md">
        {#snippet header()}
          <h3
            class="text-base font-semibold text-red-100 flex items-center gap-2"
          >
            <svg
              class="w-4 h-4 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
              />
            </svg>
            Calidad del Stream
          </h3>
        {/snippet}

        <div class="space-y-2">
          {#each qualities as quality}
            <button
              onclick={() => (selectedQuality = quality.id)}
              disabled={isStreaming}
              class="w-full p-2.5 rounded-lg border transition-all text-left {selectedQuality ===
              quality.id
                ? 'bg-red-500/20 border-red-500/50 shadow-lg shadow-red-500/20'
                : 'bg-slate-800/50 border-red-500/20 hover:border-red-500/40'} disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-sm font-semibold text-red-100">
                    {quality.name}
                  </p>
                  <p class="text-xs text-red-100/60">{quality.fps}</p>
                </div>
                {#if selectedQuality === quality.id}
                  <svg
                    class="w-4 h-4 text-red-400 flex-shrink-0"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                      clip-rule="evenodd"
                    />
                  </svg>
                {/if}
              </div>
            </button>
          {/each}
        </div>
      </Card>

      <!-- Confidence Threshold -->
      <Card variant="glass" padding="md">
        {#snippet header()}
          <h3
            class="text-base font-semibold text-red-100 flex items-center gap-2"
          >
            <svg
              class="w-4 h-4 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
              />
            </svg>
            Umbral de Confianza
          </h3>
        {/snippet}

        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-sm text-red-100/80">Mínimo</span>
            <span class="text-lg font-bold text-red-100"
              >{(confidenceThreshold * 100).toFixed(0)}%</span
            >
          </div>
          <input
            type="range"
            min="0"
            max="1"
            step="0.05"
            bind:value={confidenceThreshold}
            disabled={isStreaming}
            class="w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-red-500 disabled:opacity-50 disabled:cursor-not-allowed"
          />
          <div class="flex justify-between text-xs text-red-100/60">
            <span>0%</span>
            <span>50%</span>
            <span>100%</span>
          </div>
        </div>
      </Card>
    </div>
  </div>
</div>

<!-- Confirmation Dialog -->
<ConfirmationDialog
  bind:open={showStopDialog}
  variant="warning"
  title="Detener Monitoreo"
  message="¿Estás seguro de que deseas detener el monitoreo en tiempo real? Se perderá la conexión con el stream."
  confirmText="Detener"
  cancelText="Continuar"
  onConfirm={confirmStopStreaming}
/>

<ConfirmationDialog
  bind:open={showConflictDialog}
  title="Análisis en curso"
  message="Ya existe un análisis activo en otra pestaña. Debes detenerlo antes de iniciar uno nuevo."
  confirmText="Entendido"
  variant="danger"
  onConfirm={() => {
    showConflictDialog = false;
  }}
  cancelText=""
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

  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }

  .custom-scrollbar::-webkit-scrollbar-track {
    background: rgba(30, 41, 59, 0.5);
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
