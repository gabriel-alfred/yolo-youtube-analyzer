<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-shell";
  import { page } from "$app/stores";
  import Button from "$lib/components/ui/Button.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import ConfirmationDialog from "$lib/components/ui/ConfirmationDialog.svelte";
  import {
    COCO_CLASSES,
    DEFAULT_MODELS,
    getDownloadedModels,
    mergeModelsWithDownloads,
    type Model,
  } from "$lib/models";

  // Colores predefinidos
  const PRESET_COLORS = [
    "#ef4444",
    "#f97316",
    "#f59e0b",
    "#eab308",
    "#84cc16",
    "#22c55e",
    "#10b981",
    "#14b8a6",
    "#06b6d4",
    "#0ea5e9",
    "#3b82f6",
    "#6366f1",
    "#8b5cf6",
    "#a855f7",
    "#d946ef",
    "#ec4899",
    "#f43f5e",
  ];

  let models = $state<Model[]>([]);
  let availableModels = $derived(models.filter((m) => m.downloaded));

  let youtubeUrl = $state("");
  let selectedModel = $state("");
  let frames = $state(1);
  let quality = $state(80);
  let minPrecision = $state(25);
  let selectedDevice = $state("cpu");
  let selectedClasses = $state<Record<string, boolean>>(
    Object.fromEntries(COCO_CLASSES.map((c) => [c, true])),
  );
  let classColors = $state<Record<string, string>>(
    Object.fromEntries(
      COCO_CLASSES.map((c, i) => [c, PRESET_COLORS[i % PRESET_COLORS.length]]),
    ),
  );

  let analyzing = $state(false);
  let progress = $state(0);
  let statusMessage = $state("");
  let resultPath = $state<string | null>(null);
  let previewUrl = $state<string | null>(null);
  let modelDropdownOpen = $state(false);

  // Video player en vivo
  let showLivePlayer = $state(false);
  let liveFrameData = $state<string | null>(null);
  let showConflictDialog = $state(false);

  // Buffer y control de fpsntFrameNumber = $state(0);
  let currentFrameNumber = $state(0);
  let analysisComplete = $state(false);

  let unlistenProgress: (() => void) | null = null;
  let unlistenError: (() => void) | null = null;

  async function loadModels() {
    const downloaded = await getDownloadedModels();
    models = mergeModelsWithDownloads(DEFAULT_MODELS, downloaded);
    if (availableModels.length > 0 && !selectedModel) {
      selectedModel = availableModels[0].name;
    }
  }

  function toggleAllClasses() {
    const allSelected = Object.values(selectedClasses).every((v) => v);
    selectedClasses = Object.fromEntries(
      COCO_CLASSES.map((c) => [c, !allSelected]),
    );
  }

  function toggleClass(className: string) {
    selectedClasses[className] = !selectedClasses[className];
  }

  function extractYoutubeId(url: string): string | null {
    const regExp =
      /^.*(youtu.be\/|v\/|u\/\w\/|embed\/|watch\?v=|&v=)([^#&?]*).*/;
    const match = url.match(regExp);
    return match && match[2].length === 11 ? match[2] : null;
  }

  function getStatusMessage(status: string): string {
    const messages: Record<string, string> = {
      starting: "Iniciando...",
      downloading: "Descargando video...",
      download_complete: "Descarga completada",
      loading_model: "Cargando modelo...",
      opening_video: "Abriendo video...",
      preparing_output: "Preparando salida...",
      analyzing: "Analizando video...",
      saving: "Guardando resultado...",
      complete: "¡Análisis completado!",
      error: "Error en el análisis",
    };
    return messages[status] || status;
  }

  $effect(() => {
    const id = extractYoutubeId(youtubeUrl);
    if (id) {
      previewUrl = `https://img.youtube.com/vi/${id}/maxresdefault.jpg`;
    } else {
      previewUrl = null;
    }
  });

  async function handleAnalyze() {
    if (!youtubeUrl || !selectedModel) return;

    // Resetear todos los estados
    analyzing = true;
    showLivePlayer = true;
    analysisComplete = false;
    progress = 0;
    statusMessage = "Iniciando análisis...";
    resultPath = null;
    currentFrameNumber = 0;
    liveFrameData = null;

    try {
      // Construir string de clases (índices separados por coma)
      const classesToDetect = Object.entries(selectedClasses)
        .filter(([_, selected]) => selected)
        .map(([cls, _]) => COCO_CLASSES.indexOf(cls))
        .join(",");

      // Encontrar el objeto del modelo para obtener el nombre de archivo correcto
      const modelObj = models.find((m) => m.name === selectedModel);
      const modelFileName = modelObj ? modelObj.fileName : selectedModel;

      console.log("Starting analysis with:", {
        url: youtubeUrl,
        modelName: modelFileName,
        conf: minPrecision / 100.0,
        classes: classesToDetect,
        device: selectedDevice,
        frames: frames,
        quality: quality,
      });

      // Resetear todos los estados
      analysisComplete = false;
      progress = 0;
      statusMessage = "Iniciando análisis...";
      resultPath = null;
      currentFrameNumber = 0;
      liveFrameData = null;

      // await checkFileExists(modelFileName); // This function is not defined in the provided code

      // Descarga de modelo si es necesario (ya manejado en backend pero bueno verificar)
      // ...

      await invoke("start_video_analysis", {
        url: youtubeUrl,
        modelName: modelFileName,
        conf: minPrecision / 100.0,
        classes: classesToDetect.length > 0 ? classesToDetect : null,
        device: selectedDevice,
        frames: frames,
        quality: quality,
      });

      // Only now enables the listener and player
      analyzing = true;
      showLivePlayer = true;
    } catch (error) {
      console.error("Error starting analysis:", error);

      const errorMsg = String(error);
      if (errorMsg.includes("Ya hay un análisis en curso")) {
        showConflictDialog = true;
        conflictDialogMessage =
          "Ya existe un análisis activo en otra pestaña. Debes detenerlo antes de iniciar uno nuevo.";
        conflictDialogConfirmText = "Entendido";
        conflictDialogVariant = "danger";
        conflictDialogCancelText = "";
        // alert("⚠️ Ya hay un análisis en curso.\n\nPor favor, detenlo antes de iniciar uno nuevo.");
      }

      statusMessage = `Error: ${error}`;
      analyzing = false;
      showLivePlayer = false;
    }
  }

  async function openResult() {
    if (resultPath) {
      // Obtener el directorio del archivo para abrir la ubicación
      // Soporta tanto barras / como invertidas \
      const directory = resultPath.replace(/[\\/][^\\/]*$/, "");
      console.log("Opening directory:", directory);
      try {
        await open(directory);
      } catch (e) {
        console.error("Error opening directory:", e);
        // Fallback: intentar abrir el archivo mismo si falla el directorio
        await open(resultPath);
      }
    }
  }

  function resetAnalyzer() {
    showLivePlayer = false;
    analyzing = false;
    analysisComplete = false;
    progress = 0;
    statusMessage = "";
    resultPath = null;
    currentFrameNumber = 0;
    liveFrameData = null;
  }

  async function checkActiveAnalysis() {
    try {
      const activeSession = await invoke("get_active_analysis");
      if (activeSession) {
        console.log("Found active session:", activeSession);
        const { config } = activeSession as any;

        // Only restore if it's a video analysis
        if (config.mode && config.mode !== "video") {
          return;
        }

        // Restaurar estado
        youtubeUrl = config.url;
        selectedModel = config.model_name; // Note: this might need mapping back to display name if different
        minPrecision = config.conf * 100;
        selectedDevice = config.device;
        frames = config.frames;
        quality = config.quality;

        // Restaurar clases
        if (config.classes) {
          const activeClasses = config.classes.split(",").map(Number);
          // Reset all to false first
          for (const key in selectedClasses) selectedClasses[key] = false;
          // Set active ones to true
          activeClasses.forEach((idx: number) => {
            if (idx >= 0 && idx < COCO_CLASSES.length) {
              selectedClasses[COCO_CLASSES[idx]] = true;
            }
          });
        }

        analyzing = true;
        showLivePlayer = true;
        statusMessage = "Reconectando con análisis en curso...";
      }
    } catch (e) {
      console.error("Error checking active analysis:", e);
    }
  }

  async function handleStop() {
    try {
      await invoke("stop_video_analysis");
      analyzing = false;
      statusMessage = "Análisis detenido por el usuario";
      // Opcional: Mantener el player visible pero indicar que paró
    } catch (e) {
      console.error("Error stopping analysis:", e);
      statusMessage = `Error al detener: ${e}`;
    }
  }

  onMount(async () => {
    await loadModels();
    await checkActiveAnalysis();

    if (!analyzing) {
      const params = $page.url.searchParams;
      if (params.has("url")) {
        youtubeUrl = params.get("url") || "";
        const modelParam = params.get("model");
        if (modelParam) {
          // Try to match with available models
          const found = models.find(
            (m) => m.name === modelParam || m.fileName === modelParam,
          );
          if (found) selectedModel = found.name;
        }
        if (params.has("conf")) minPrecision = parseFloat(params.get("conf")!);
        if (params.has("frames")) frames = parseInt(params.get("frames")!);
        if (params.has("quality")) quality = parseInt(params.get("quality")!);
      }
    }

    unlistenProgress = await listen("analysis-progress", (event: any) => {
      // Guard: If not strictly analyzing, ignore events (to avoid cross-talk with Live Analysis)
      if (!analyzing && !analysisComplete) return;

      const payload = event.payload;

      console.log("Progress event:", payload);

      // Actualizar progreso SIEMPRE (incluso para frames)
      if (payload.progress !== undefined) {
        progress = payload.progress;
      }

      // Actualizar mensaje de estado
      if (payload.status) {
        const currentStatus = payload.status;

        // Manejar frames en vivo
        if (currentStatus === "frame" && payload.frame_data) {
          liveFrameData = `data:image/jpeg;base64,${payload.frame_data}`;
          currentFrameNumber = payload.frame_number || 0;
          console.log(`Received frame ${currentFrameNumber}`);
          return;
        }

        // Actualizar mensaje solo para eventos que no sean frames
        statusMessage = payload.message || getStatusMessage(currentStatus);

        // Manejar completado
        if (currentStatus === "complete" && payload.result_path) {
          resultPath = payload.result_path;

          // Debug: verificar conversión de ruta
          const convertedPath = convertFileSrc(resultPath!);
          console.log("Original path:", resultPath);
          console.log("Converted path:", convertedPath);

          analyzing = false;
          analysisComplete = true;
          progress = 100;
          statusMessage = "¡Análisis completado!";
          console.log("Analysis complete!");
          console.log("  - resultPath:", resultPath);
          console.log("  - analysisComplete:", analysisComplete);
          console.log("  - showLivePlayer:", showLivePlayer);
        }
      }
    });

    unlistenError = await listen("analysis-error", (event: any) => {
      console.error("Analysis error:", event.payload);
      const errorMsg = String(event.payload);

      // Ignorar errores no fatales de inicialización de OpenCV/FFmpeg
      if (
        errorMsg.includes("OpenH264") ||
        errorMsg.includes("FFMPEG") ||
        errorMsg.includes("tag 0x") ||
        errorMsg.includes("Incorrect library version") ||
        errorMsg.includes("Could not open codec")
      ) {
        console.warn("Ignored non-fatal error:", errorMsg);
        return;
      }

      statusMessage = `Error: ${errorMsg}`;

      // Solo detener si parece un error fatal o el script falló
      if (
        errorMsg.includes("Script failed") ||
        errorMsg.includes("Error analizando video") ||
        errorMsg.includes("Failed to spawn")
      ) {
        analyzing = false;
        showLivePlayer = false;
      }
    });
  });

  onDestroy(() => {
    if (unlistenProgress) unlistenProgress();
    if (unlistenError) unlistenError();
  });
</script>

<!-- Header -->
<div class="text-center mb-12">
  <h1
    class="text-5xl font-bold bg-gradient-to-r from-red-400 via-orange-400 to-red-500 bg-clip-text text-transparent mb-4"
  >
    Análisis de Video
  </h1>
  <p class="text-slate-300 text-lg">
    Analiza videos de YouTube con detección de objetos en tiempo real
  </p>
</div>

<div class="space-y-6">
  <!-- Video Preview (solo si NO está en live player) -->
  {#if !showLivePlayer && previewUrl}
    <Card variant="gradient" padding="lg">
      <div class="flex flex-col md:flex-row gap-4 items-end">
        <div class="flex-1 w-full">
          <Input
            label="URL del Video de YouTube"
            placeholder="https://www.youtube.com/watch?v=..."
            bind:value={youtubeUrl}
            fullWidth
          >
            {#snippet icon()}
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"
                />
              </svg>
            {/snippet}
          </Input>
        </div>
        <div class="w-full md:w-auto">
          <Button
            variant={analyzing ? "danger" : "primary"}
            size="lg"
            loading={analyzing && !showLivePlayer}
            disabled={(!youtubeUrl || !selectedModel) && !analyzing}
            onclick={analyzing ? handleStop : handleAnalyze}
            class="w-full md:w-auto min-w-[150px]"
          >
            {analyzing ? "Detener Análisis" : "Analizar Video"}
          </Button>
        </div>
      </div>

      <!-- Video Preview -->
      <div class="mt-6 flex justify-center">
        <div class="w-full max-w-md">
          <h3 class="text-sm font-semibold text-red-200 mb-3">Vista Previa</h3>
          <div
            class="relative rounded-lg overflow-hidden border border-red-500/30 shadow-lg shadow-red-500/20"
          >
            <img
              src={previewUrl}
              alt="YouTube Preview"
              class="w-full h-auto"
              onerror={(e) => {
                (e.currentTarget as HTMLImageElement).src =
                  `https://img.youtube.com/vi/${extractYoutubeId(youtubeUrl)}/hqdefault.jpg`;
              }}
            />
          </div>
        </div>
      </div>
    </Card>
  {/if}

  <!-- Live Video Player -->
  {#if showLivePlayer}
    <Card variant="gradient" padding="lg">
      <div class="space-y-4">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-xl font-bold text-red-100">
            {analysisComplete ? "Video Analizado" : "Análisis en Vivo"}
          </h2>
          <div class="flex items-center gap-3">
            {#if !analysisComplete}
              <span class="text-sm text-red-300"
                >Frame: {currentFrameNumber}</span
              >
            {/if}
          </div>
        </div>

        <!-- Video Display - Live o Final -->
        {#if !analysisComplete}
          <!-- Live Analysis View -->
          <div
            class="relative rounded-lg overflow-hidden border-2 border-red-500/50 bg-black aspect-video flex items-center justify-center shadow-2xl shadow-red-500/30"
          >
            {#if liveFrameData}
              <img
                src={liveFrameData}
                alt="Live analysis"
                class="w-full h-full object-contain"
              />
            {:else}
              <div class="flex flex-col items-center justify-center gap-3">
                <svg
                  class="w-12 h-12 text-red-500 animate-pulse"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                  />
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                <p class="text-red-300 font-medium">
                  Esperando primeros frames...
                </p>
                <p class="text-red-400/60 text-sm">{statusMessage}</p>
              </div>
            {/if}
          </div>

          <!-- Progress Bar (solo durante análisis) -->
          <div class="space-y-2">
            <div class="flex justify-between text-sm text-red-200">
              <span>{statusMessage}</span>
              <span>{Math.round(progress)}%</span>
            </div>
            <div
              class="h-2 bg-slate-900/50 rounded-full overflow-hidden border border-red-500/20"
            >
              <div
                class="h-full bg-gradient-to-r from-red-600 to-red-400 transition-all duration-300 relative"
                style="width: {progress}%"
              >
                <div
                  class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent animate-shimmer"
                ></div>
              </div>
            </div>
          </div>
        {:else if resultPath}
          <!-- Completed Analysis View -->
          <div class="space-y-4">
            <div
              class="p-4 bg-green-900/20 border border-green-500/30 rounded-lg"
            >
              <p class="text-green-100 font-semibold">
                ✓ Análisis completado exitosamente
              </p>
              <p class="text-green-200/80 text-sm mt-1">
                Archivo: {resultPath.split(/[\\/]/).pop()}
              </p>
            </div>

            <div
              class="relative rounded-lg overflow-hidden border-2 border-green-500/50 bg-black shadow-2xl shadow-green-500/30"
            >
              {#key resultPath}
                <!-- svelte-ignore a11y_media_has_caption -->
                <video
                  src={convertFileSrc(resultPath!)}
                  controls
                  autoplay={false}
                  class="w-full h-auto max-h-[70vh]"
                  preload="auto"
                  onloadstart={(e) => {
                    const src = convertFileSrc(resultPath!);
                    console.log("Video starting to load:", src);
                    console.log("Original path:", resultPath);

                    const video = e.currentTarget as HTMLVideoElement;
                    // Asegurar que el src esté correctamente establecido
                    if (video.src !== src) {
                      video.src = src;
                    }
                  }}
                  onloadedmetadata={(e) => {
                    const video = e.currentTarget as HTMLVideoElement;
                    console.log("✓ Video metadata loaded");
                    console.log("  Duration:", video.duration);
                    console.log(
                      "  Dimensions:",
                      video.videoWidth,
                      "x",
                      video.videoHeight,
                    );
                  }}
                  onloadeddata={() => {
                    console.log("✓ Video data loaded successfully!");
                  }}
                  onerror={(e) => {
                    const video = e.currentTarget as HTMLVideoElement;
                    console.error("❌ Video error:", e);
                    console.error("  Video src:", video.src);
                    console.error("  Original path:", resultPath);
                    console.error("  Error code:", video.error?.code);
                    console.error("  Error message:", video.error?.message);

                    // Mapear códigos de error
                    const errorMessages: Record<number, string> = {
                      1: "MEDIA_ERR_ABORTED - Carga abortada por el usuario",
                      2: "MEDIA_ERR_NETWORK - Error de red durante la descarga",
                      3: "MEDIA_ERR_DECODE - Error decodificando el video",
                      4: "MEDIA_ERR_SRC_NOT_SUPPORTED - Formato no soportado o archivo corrupto",
                    };

                    const errorCode = video.error?.code || 0;
                    console.error(
                      "  Descripción:",
                      errorMessages[errorCode] || "Error desconocido",
                    );

                    // Mostrar mensaje al usuario
                    if (errorCode === 4) {
                      statusMessage =
                        "Error: El formato del video no es compatible. Instala FFmpeg para generar videos compatibles.";
                    }
                  }}
                  oncanplay={() => {
                    console.log("✓ Video can play!");
                  }}
                  oncanplaythrough={() => {
                    console.log("✓ Video fully loaded and ready!");
                  }}
                >
                  Tu navegador no soporta la reproducción de video.
                  <p class="text-red-300 p-4">
                    El video fue generado pero tu navegador no puede
                    reproducirlo. Haz clic en "Abrir ubicación del archivo" para
                    verlo en un reproductor externo.
                  </p>
                </video>
              {/key}
            </div>

            <div class="flex justify-end gap-2">
              <Button variant="outline" size="sm" onclick={openResult}>
                📁 Abrir ubicación del archivo
              </Button>
              <Button variant="ghost" size="sm" onclick={resetAnalyzer}>
                Nuevo Análisis
              </Button>
            </div>
          </div>
        {/if}
      </div>
    </Card>
  {/if}

  <!-- Configuration Cards (solo si no está analizando o en live view) -->
  {#if !showLivePlayer}
    <!-- Input Section (sin preview) -->
    {#if !previewUrl}
      <Card variant="gradient" padding="lg">
        <div class="flex flex-col gap-4">
          <Input
            label="URL del Video de YouTube"
            placeholder="https://www.youtube.com/watch?v=..."
            bind:value={youtubeUrl}
            fullWidth
          >
            {#snippet icon()}
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"
                />
              </svg>
            {/snippet}
          </Input>
          <Button
            variant={analyzing ? "danger" : "primary"}
            size="lg"
            loading={analyzing && !showLivePlayer}
            disabled={(!youtubeUrl || !selectedModel) && !analyzing}
            onclick={analyzing ? handleStop : handleAnalyze}
            class="w-full"
          >
            {analyzing ? "Detener Análisis" : "Analizar Video"}
          </Button>
        </div>
      </Card>
    {/if}

    <!-- Model and Parameters -->
    <Card variant="default" padding="lg" class="overflow-visible">
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
              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z"
            />
          </svg>
          Configuración del Modelo
        </h2>
      {/snippet}

      <div class="space-y-5">
        <!-- Model Selector -->
        <div class="relative">
          <label
            for="model-selector"
            class="block text-sm font-medium text-red-200 mb-2"
          >
            Modelo YOLO
          </label>
          <div class="relative z-50">
            <button
              id="model-selector"
              onclick={() => (modelDropdownOpen = !modelDropdownOpen)}
              class="w-full px-4 py-2.5 bg-slate-900/60 border border-red-500/30 rounded-lg text-left text-red-50 hover:border-red-500/50 transition-all flex items-center justify-between focus:outline-none focus:border-red-500"
            >
              <span
                >{selectedModel ||
                  (availableModels.length > 0
                    ? "Seleccionar modelo"
                    : "No hay modelos descargados")}</span
              >
              <svg
                class="w-5 h-5 text-red-400 transition-transform {modelDropdownOpen
                  ? 'rotate-180'
                  : ''}"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </button>

            {#if modelDropdownOpen}
              <div
                class="absolute top-full left-0 right-0 mt-2 border-2 border-red-500/50 rounded-lg shadow-2xl z-[100]"
                style="background-color: #1e293b;"
              >
                {#if availableModels.length === 0}
                  <div class="px-4 py-3 text-red-300 text-sm">
                    No hay modelos descargados. Ve a la pestaña Modelos para
                    descargar uno.
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
                      onmouseenter={(e) =>
                        (e.currentTarget.style.backgroundColor = "#991b1b30")}
                      onmouseleave={(e) =>
                        (e.currentTarget.style.backgroundColor = "#1e293b")}
                    >
                      <span class="font-medium text-base">{model.name}</span>
                      {#if selectedModel === model.name}
                        <svg
                          class="w-5 h-5 text-red-500"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M5 13l4 4L19 7"
                          />
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

          <!-- Device Selection -->
          <div>
            <label class="block text-sm font-medium text-red-200 mb-2">
              Dispositivo de Procesamiento
            </label>
            <select
              bind:value={selectedDevice}
              class="w-full px-3 py-2.5 bg-slate-900/60 border border-red-500/30 rounded-lg text-red-100 hover:border-red-500/50 focus:border-red-500 transition-all focus:outline-none"
            >
              <option value="cpu">CPU (Más lento, compatible)</option>
              <option value="cuda">GPU NVIDIA (Más rápido)</option>
              <option value="mps">GPU Apple Silicon (MacBook M1/M2)</option>
            </select>
            <p class="text-xs text-red-300/60 mt-1">
              {#if selectedDevice === "cpu"}
                Recomendado si no tienes GPU compatible
              {:else if selectedDevice === "cuda"}
                Requiere NVIDIA GPU y CUDA instalado
              {:else if selectedDevice === "mps"}
                Solo para MacBooks con chips Apple
              {/if}
            </p>
          </div>
        </div>
      </div>
    </Card>

    <!-- Classes Selection -->
    <Card variant="default" padding="lg">
      {#snippet header()}
        <div class="flex items-center justify-between">
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
                d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
              />
            </svg>
            Clases a Detectar
          </h2>
          <Button variant="ghost" size="sm" onclick={toggleAllClasses}>
            {Object.values(selectedClasses).every((v) => v)
              ? "Deseleccionar Todo"
              : "Seleccionar Todo"}
          </Button>
        </div>
      {/snippet}

      <div class="max-h-96 overflow-y-auto pr-2 space-y-2 custom-scrollbar">
        {#each COCO_CLASSES as className}
          <div
            class="flex items-center gap-3 p-3 bg-slate-900/40 border border-red-500/20 rounded-lg hover:border-red-500/40 transition-all"
          >
            <!-- Checkbox -->
            <input
              type="checkbox"
              checked={selectedClasses[className]}
              onchange={() => toggleClass(className)}
              class="w-5 h-5 rounded border-red-500/30 text-red-500 focus:ring-red-500/50 bg-slate-800 cursor-pointer flex-shrink-0"
            />

            <!-- Class Name -->
            <span
              class="flex-1 text-sm font-medium {selectedClasses[className]
                ? 'text-red-100'
                : 'text-red-300/50'}"
            >
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
  {/if}
</div>

<style>
  @keyframes shimmer {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(100%);
    }
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
