<script lang="ts">
  import Button from "$lib/components/ui/Button.svelte";
  import Card from "$lib/components/ui/Card.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import LoadingSpinner from "$lib/components/ui/LoadingSpinner.svelte";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";

  interface VideoAnalysis {
    id: string;
    videoUrl: string;
    thumbnail: string;
    title: string;
    duration: string;
    analyzedDate: string;
    totalObjects: number;
    detectedClasses: string[];
    model: string;
    quality: string;
    framesInterval: number;
    minConfidence: number;
    processingTime: string;
    resultPath: string;
    status?: string;
  }

  let analyses = $state<VideoAnalysis[]>([]);
  let searchQuery = $state("");
  let sortBy = $state<"date" | "objects" | "duration">("date");
  let loading = $state(true);

  $effect(() => {
    loadAnalyses();
  });

  async function loadAnalyses() {
    try {
      loading = true;
      analyses = await invoke("get_all_analysis_results");
    } catch (error) {
      console.error("Error loading analyses:", error);
    } finally {
      loading = false;
    }
  }

  // Filtrar y ordenar análisis
  let filteredAnalyses = $derived.by(() => {
    let filtered = analyses.filter(
      (a) =>
        a.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        a.detectedClasses.some((c) =>
          c.toLowerCase().includes(searchQuery.toLowerCase()),
        ),
    );

    switch (sortBy) {
      case "date":
        return filtered.sort(
          (a, b) =>
            new Date(b.analyzedDate).getTime() -
            new Date(a.analyzedDate).getTime(),
        );
      case "objects":
        return filtered.sort((a, b) => b.totalObjects - a.totalObjects);
      case "duration":
        return filtered.sort((a, b) => {
          const durationA = parseDuration(a.duration);
          const durationB = parseDuration(b.duration);
          return durationB - durationA;
        });
      default:
        return filtered;
    }
  });

  function parseDuration(duration: string): number {
    const parts = duration.split(":").map(Number);
    if (parts.length === 2) {
      return parts[0] * 60 + parts[1];
    }
    return parts[0] * 3600 + parts[1] * 60 + parts[2];
  }

  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    const now = new Date();
    const diffTime = Math.abs(now.getTime() - date.getTime());
    const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return "Hoy";
    if (diffDays === 1) return "Ayer";
    if (diffDays < 7) return `Hace ${diffDays} días`;

    return date.toLocaleDateString("es-ES", {
      day: "numeric",
      month: "short",
      year: "numeric",
    });
  }

  function handleCardClick(id: string) {
    goto(`/results/${id}`);
  }

  function handleDelete(id: string, event: Event) {
    event.stopPropagation();
    // Implementar lógica de eliminación
    analyses = analyses.filter((a) => a.id !== id);
  }

  function handleRestart(analysis: VideoAnalysis, event: Event) {
    event.stopPropagation();
    const params = new URLSearchParams({
      url: analysis.videoUrl,
      model: analysis.model,
      conf: (analysis.minConfidence * 100).toString(),
      frames: analysis.framesInterval.toString(),
      quality: analysis.quality.replace("p", ""),
    });
    goto(`/analysis?${params.toString()}`);
  }
</script>

<!-- Header -->
<div class="text-center mb-12">
  <h1
    class="text-5xl font-bold bg-gradient-to-r from-red-400 via-orange-400 to-red-500 bg-clip-text text-transparent mb-4"
  >
    Análisis Realizados
  </h1>
  <p class="text-slate-300 text-lg">
    Explora todos los videos que has analizado con YOLO
  </p>
</div>

<div class="max-w-7xl mx-auto space-y-6">
  <!-- Filters and Search -->
  <Card variant="default" padding="lg">
    <div class="flex flex-col lg:flex-row gap-4">
      <!-- Search -->
      <div class="flex-1">
        <Input
          bind:value={searchQuery}
          type="search"
          placeholder="Buscar por título o clases detectadas..."
          fullWidth={true}
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
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
          {/snippet}
        </Input>
      </div>

      <!-- Sort -->
      <div class="flex items-center gap-3">
        <span class="text-sm text-red-300 whitespace-nowrap">Ordenar por:</span>
        <div class="relative">
          <select
            bind:value={sortBy}
            class="appearance-none pl-4 pr-9 py-2.5 bg-slate-900/60 border border-red-500/30 rounded-lg text-red-50 focus:border-red-500 focus:outline-none transition-all cursor-pointer hover:bg-slate-900/80"
          >
            <option value="date">Fecha</option>
            <option value="objects">Objetos detectados</option>
            <option value="duration">Duración</option>
          </select>
          <svg
            class="w-4 h-4 absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none text-red-400"
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
        </div>
      </div>
    </div>

    <!-- Stats -->
    <div
      class="grid grid-cols-2 lg:grid-cols-4 gap-4 mt-6 pt-6 border-t border-red-500/20"
    >
      <div class="text-center">
        <p class="text-3xl font-bold text-red-400">{analyses.length}</p>
        <p class="text-sm text-red-300/70 mt-1">Videos analizados</p>
      </div>
      <div class="text-center">
        <p class="text-3xl font-bold text-orange-400">
          {analyses
            .reduce((sum, a) => sum + a.totalObjects, 0)
            .toLocaleString()}
        </p>
        <p class="text-sm text-red-300/70 mt-1">Objetos detectados</p>
      </div>
      <div class="text-center">
        <p class="text-3xl font-bold text-red-400">
          {new Set(analyses.flatMap((a) => a.detectedClasses)).size}
        </p>
        <p class="text-sm text-red-300/70 mt-1">Clases únicas</p>
      </div>
      <div class="text-center">
        <p class="text-3xl font-bold text-orange-400">
          {new Set(analyses.map((a) => a.model)).size}
        </p>
        <p class="text-sm text-red-300/70 mt-1">Modelos usados</p>
      </div>
    </div>
  </Card>

  <!-- Loading State -->
  {#if loading}
    <div class="flex justify-center py-20">
      <LoadingSpinner size="lg" message="Cargando análisis..." />
    </div>
  {:else if filteredAnalyses.length === 0}
    <!-- Empty State -->
    <Card variant="gradient" padding="lg">
      <div class="text-center py-12">
        <div
          class="w-20 h-20 mx-auto mb-6 rounded-full bg-red-500/20 flex items-center justify-center"
        >
          <svg
            class="w-10 h-10 text-red-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </div>
        <h3 class="text-2xl font-bold text-red-100 mb-2">
          No se encontraron resultados
        </h3>
        <p class="text-red-300/70 mb-6">
          {searchQuery
            ? "Intenta con otros términos de búsqueda"
            : "Aún no has analizado ningún video"}
        </p>
        {#if !searchQuery}
          <Button variant="primary" onclick={() => goto("/analysis")}>
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
                  d="M12 4v16m8-8H4"
                />
              </svg>
            {/snippet}
            Analizar un video
          </Button>
        {/if}
      </div>
    </Card>
  {:else}
    <!-- Results Grid -->
    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each filteredAnalyses as analysis (analysis.id)}
        <Card
          variant="elevated"
          padding="none"
          hoverable={true}
          class="cursor-pointer group"
        >
          <button
            onclick={() => handleCardClick(analysis.id)}
            class="w-full text-left"
          >
            <!-- Thumbnail -->
            <div class="relative overflow-hidden">
              <img
                src={analysis.thumbnail}
                alt={analysis.title}
                class="w-full h-48 object-cover transition-transform duration-500 group-hover:scale-110"
              />

              <!-- Duration Badge -->
              <div
                class="absolute bottom-3 right-3 px-2 py-1 bg-slate-950/90 backdrop-blur-sm rounded text-xs font-semibold text-white"
              >
                {analysis.duration}
              </div>

              <!-- Date Badge -->
              <div
                class="absolute top-3 left-3 px-2 py-1 bg-slate-950/90 backdrop-blur-sm rounded text-xs font-medium text-red-200"
              >
                {formatDate(analysis.analyzedDate)}
              </div>

              <!-- Status Badge -->
              {#if analysis.status && analysis.status !== "complete"}
                <div
                  class="absolute top-3 right-3 px-2 py-1 rounded text-xs font-bold uppercase
                  {analysis.status === 'analyzing'
                    ? 'bg-blue-500/90 text-white animate-pulse'
                    : 'bg-yellow-500/90 text-black'}"
                >
                  {analysis.status === "analyzing" ? "Analizando" : "Detenido"}
                </div>
              {/if}

              <!-- Hover Overlay -->
              <div
                class="absolute inset-0 bg-gradient-to-t from-slate-950/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-center justify-center"
              >
                <div
                  class="transform translate-y-4 group-hover:translate-y-0 transition-transform duration-300"
                >
                  <div
                    class="w-14 h-14 rounded-full bg-red-500/90 flex items-center justify-center"
                  >
                    <svg
                      class="w-7 h-7 text-white"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                      />
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                      />
                    </svg>
                  </div>
                </div>
              </div>
            </div>

            <!-- Content -->
            <div class="p-5 space-y-4">
              <!-- Title -->
              <h3
                class="font-bold text-red-100 line-clamp-2 text-lg group-hover:text-red-300 transition-colors"
              >
                {analysis.title}
              </h3>

              <!-- Stats Grid -->
              <div class="grid grid-cols-2 gap-3 text-sm">
                <div class="flex items-center gap-2">
                  <div
                    class="w-8 h-8 rounded-lg bg-red-500/20 flex items-center justify-center flex-shrink-0"
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
                        d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
                      />
                    </svg>
                  </div>
                  <div>
                    <p class="text-xs text-red-300/60">Objetos</p>
                    <p class="font-semibold text-red-100">
                      {analysis.totalObjects.toLocaleString()}
                    </p>
                  </div>
                </div>

                <div class="flex items-center gap-2">
                  <div
                    class="w-8 h-8 rounded-lg bg-orange-500/20 flex items-center justify-center flex-shrink-0"
                  >
                    <svg
                      class="w-4 h-4 text-orange-400"
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
                  </div>
                  <div>
                    <p class="text-xs text-red-300/60">Modelo</p>
                    <p class="font-semibold text-red-100">{analysis.model}</p>
                  </div>
                </div>
              </div>

              <!-- Classes -->
              <div>
                <p class="text-xs text-red-300/60 mb-2">Clases detectadas:</p>
                <div class="flex flex-wrap gap-1.5">
                  {#each analysis.detectedClasses.slice(0, 4) as className}
                    <span
                      class="px-2 py-0.5 bg-slate-800/60 border border-red-500/20 rounded text-xs text-red-200"
                    >
                      {className}
                    </span>
                  {/each}
                  {#if analysis.detectedClasses.length > 4}
                    <span
                      class="px-2 py-0.5 bg-slate-800/60 border border-red-500/20 rounded text-xs text-red-300/70"
                    >
                      +{analysis.detectedClasses.length - 4}
                    </span>
                  {/if}
                </div>
              </div>

              <!-- Config Details -->
              <div
                class="flex items-center justify-between text-xs text-red-300/60 pt-3 border-t border-red-500/20"
              >
                <div class="flex items-center gap-3">
                  <span title="Calidad">📹 {analysis.quality}</span>
                  <span title="Intervalo de frames"
                    >🎞️ {analysis.framesInterval}f</span
                  >
                  <span title="Confianza mínima"
                    >🎯 {(analysis.minConfidence * 100).toFixed(0)}%</span
                  >
                </div>
              </div>
            </div>
          </button>

          <!-- Action Buttons -->
          <div class="px-5 pb-5 flex gap-2">
            <Button
              variant="primary"
              size="sm"
              class="flex-1"
              disabled={!!(analysis.status && analysis.status !== "complete")}
              onclick={() => handleCardClick(analysis.id)}
            >
              Ver detalles
            </Button>

            <Button
              variant="secondary"
              size="sm"
              class="flex-1"
              onclick={(e) => handleRestart(analysis, e)}
            >
              🔄 Reiniciar
            </Button>

            <Button
              variant="danger"
              size="sm"
              onclick={(e) => handleDelete(analysis.id, e)}
            >
              {#snippet icon()}
                <svg
                  class="w-4 h-4"
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
            </Button>
          </div>
        </Card>
      {/each}
    </div>
  {/if}
</div>

<style>
  .line-clamp-2 {
    display: -webkit-box;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
