<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';

  interface Detection {
    frameNumber: number;
    timestamp: string;
    class: string;
    confidence: number;
    bbox: {
      x: number;
      y: number;
      width: number;
      height: number;
    };
  }

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
    detections: Detection[];
    videoId: string;
    fps: number;
  }

  // Mock data - en producción vendría del backend usando el ID
  const mockAnalyses: Record<string, VideoAnalysis> = {
    '1': {
      id: '1',
      videoUrl: 'https://youtube.com/watch?v=dQw4w9WgXcQ',
      videoId: 'dQw4w9WgXcQ',
      thumbnail: 'https://picsum.photos/seed/video1/640/360',
      title: 'Tutorial de React - Componentes y Hooks',
      duration: '15:42',
      analyzedDate: '2024-11-01T14:30:00',
      totalObjects: 1247,
      detectedClasses: ['person', 'laptop', 'keyboard', 'mouse', 'book'],
      model: 'YOLOv11s',
      quality: '720p',
      framesInterval: 30,
      minConfidence: 0.45,
      processingTime: '2m 15s',
      fps: 30,
      detections: [
        { frameNumber: 30, timestamp: '00:01', class: 'person', confidence: 0.92, bbox: { x: 120, y: 80, width: 200, height: 400 } },
        { frameNumber: 60, timestamp: '00:02', class: 'laptop', confidence: 0.87, bbox: { x: 300, y: 250, width: 350, height: 200 } },
        { frameNumber: 90, timestamp: '00:03', class: 'keyboard', confidence: 0.78, bbox: { x: 320, y: 380, width: 280, height: 80 } },
        { frameNumber: 120, timestamp: '00:04', class: 'person', confidence: 0.94, bbox: { x: 115, y: 75, width: 210, height: 410 } },
        { frameNumber: 150, timestamp: '00:05', class: 'mouse', confidence: 0.65, bbox: { x: 580, y: 420, width: 40, height: 60 } },
        { frameNumber: 180, timestamp: '00:06', class: 'laptop', confidence: 0.89, bbox: { x: 305, y: 255, width: 345, height: 195 } },
        { frameNumber: 210, timestamp: '00:07', class: 'book', confidence: 0.71, bbox: { x: 450, y: 350, width: 120, height: 160 } },
        { frameNumber: 240, timestamp: '00:08', class: 'person', confidence: 0.91, bbox: { x: 118, y: 82, width: 205, height: 405 } },
      ]
    },
    '2': {
      id: '2',
      videoUrl: 'https://youtube.com/watch?v=example2',
      videoId: 'example2',
      thumbnail: 'https://picsum.photos/seed/video2/640/360',
      title: 'Cooking Show - Making Pizza from Scratch',
      duration: '23:15',
      analyzedDate: '2024-10-31T09:15:00',
      totalObjects: 2891,
      detectedClasses: ['person', 'bowl', 'knife', 'pizza', 'oven', 'bottle'],
      model: 'YOLOv11m',
      quality: '1080p',
      framesInterval: 15,
      minConfidence: 0.5,
      processingTime: '5m 43s',
      fps: 30,
      detections: []
    }
  };

  let loading = $state(true);
  let analysis = $state<VideoAnalysis | null>(null);
  let error = $state('');
  let selectedTab = $state<'overview' | 'detections'>('overview');
  let classFilter = $state<string>('all');

  // Obtener ID de la URL
  $effect(() => {
    const id = $page.params.id;
    
    // Simular carga de datos
    setTimeout(() => {
      const data = mockAnalyses[id];
      if (data) {
        analysis = data;
        loading = false;
      } else {
        error = 'Análisis no encontrado';
        loading = false;
      }
    }, 500);
  });

  // Filtrar detecciones por clase
  let filteredDetections = $derived.by(() => {
    if (!analysis) return [];
    if (classFilter === 'all') return analysis.detections;
    return analysis.detections.filter(d => d.class === classFilter);
  });

  // Estadísticas por clase
  let classStats = $derived.by(() => {
    if (!analysis) return [];
    const stats = new Map<string, { count: number; avgConfidence: number }>();
    
    analysis.detections.forEach(d => {
      const current = stats.get(d.class) || { count: 0, avgConfidence: 0 };
      stats.set(d.class, {
        count: current.count + 1,
        avgConfidence: (current.avgConfidence * current.count + d.confidence) / (current.count + 1)
      });
    });
    
    return Array.from(stats.entries()).map(([className, data]) => ({
      class: className,
      ...data
    })).sort((a, b) => b.count - a.count);
  });

  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleDateString('es-ES', { 
      day: 'numeric', 
      month: 'long', 
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function getConfidenceColor(confidence: number): string {
    if (confidence >= 0.8) return 'text-green-400';
    if (confidence >= 0.6) return 'text-yellow-400';
    return 'text-orange-400';
  }

  function downloadJSON() {
    if (!analysis) return;
    const dataStr = JSON.stringify(analysis, null, 2);
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `analysis-${analysis.id}.json`;
    link.click();
    URL.revokeObjectURL(url);
  }

  function downloadCSV() {
    if (!analysis) return;
    const headers = ['Frame', 'Timestamp', 'Class', 'Confidence', 'X', 'Y', 'Width', 'Height'];
    const rows = analysis.detections.map(d => [
      d.frameNumber,
      d.timestamp,
      d.class,
      d.confidence.toFixed(2),
      d.bbox.x,
      d.bbox.y,
      d.bbox.width,
      d.bbox.height
    ]);
    
    const csv = [headers, ...rows].map(row => row.join(',')).join('\n');
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `detections-${analysis.id}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  }
</script>

{#if loading}
  <div class="flex items-center justify-center min-h-[60vh]">
    <LoadingSpinner size="lg" message="Cargando análisis..." />
  </div>
{:else if error || !analysis}
  <div class="max-w-2xl mx-auto mt-12">
    <ErrorMessage
      variant="error"
      title="Error"
      message={error || 'No se pudo cargar el análisis'}
    >
      {#snippet actions()}
        <Button variant="primary" size="sm" onclick={() => goto('/results')}>
          {#snippet icon()}
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
          {/snippet}
          Volver a resultados
        </Button>
      {/snippet}
    </ErrorMessage>
  </div>
{:else}
  <!-- Header -->
  <div class="mb-8">
    <Button variant="ghost" size="sm" onclick={() => goto('/results')} class="mb-4">
      {#snippet icon()}
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
        </svg>
      {/snippet}
      Volver a resultados
    </Button>

    <div class="flex flex-col lg:flex-row lg:items-start lg:justify-between gap-6">
      <div class="flex-1">
        <h1 class="text-4xl font-bold text-red-100 mb-3">{analysis.title}</h1>
        <div class="flex flex-wrap items-center gap-4 text-sm text-red-300/70">
          <span class="flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            {formatDate(analysis.analyzedDate)}
          </span>
          <span class="flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            {analysis.duration}
          </span>
          <span class="flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            Procesado en {analysis.processingTime}
          </span>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex flex-wrap gap-2">
        <Button variant="outline" size="sm" onclick={downloadJSON}>
          {#snippet icon()}
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          {/snippet}
          JSON
        </Button>
        <Button variant="outline" size="sm" onclick={downloadCSV}>
          {#snippet icon()}
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          {/snippet}
          CSV
        </Button>
      </div>
    </div>
  </div>

  <div class="max-w-7xl mx-auto space-y-6">
    <!-- Video Player -->
    <Card variant="elevated" padding="none">
      <div class="aspect-video bg-slate-950 relative overflow-hidden">
        <iframe
          class="w-full h-full"
          src="https://www.youtube.com/embed/{analysis.videoId}"
          title={analysis.title}
          frameborder="0"
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
          allowfullscreen
        ></iframe>
      </div>
    </Card>

    <!-- Stats Grid -->
    <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <Card variant="gradient" padding="lg">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 rounded-xl bg-red-500/20 flex items-center justify-center flex-shrink-0">
            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
            </svg>
          </div>
          <div>
            <p class="text-3xl font-bold text-red-100">{analysis.totalObjects.toLocaleString()}</p>
            <p class="text-sm text-red-300/70">Objetos detectados</p>
          </div>
        </div>
      </Card>

      <Card variant="gradient" padding="lg">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 rounded-xl bg-orange-500/20 flex items-center justify-center flex-shrink-0">
            <svg class="w-6 h-6 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
            </svg>
          </div>
          <div>
            <p class="text-3xl font-bold text-red-100">{analysis.detectedClasses.length}</p>
            <p class="text-sm text-red-300/70">Clases únicas</p>
          </div>
        </div>
      </Card>

      <Card variant="gradient" padding="lg">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 rounded-xl bg-blue-500/20 flex items-center justify-center flex-shrink-0">
            <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
            </svg>
          </div>
          <div>
            <p class="text-3xl font-bold text-red-100">{analysis.model}</p>
            <p class="text-sm text-red-300/70">Modelo YOLO</p>
          </div>
        </div>
      </Card>

      <Card variant="gradient" padding="lg">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 rounded-xl bg-green-500/20 flex items-center justify-center flex-shrink-0">
            <svg class="w-6 h-6 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <div>
            <p class="text-3xl font-bold text-red-100">{(analysis.minConfidence * 100).toFixed(0)}%</p>
            <p class="text-sm text-red-300/70">Confianza mínima</p>
          </div>
        </div>
      </Card>
    </div>

    <!-- Tabs -->
    <div class="flex gap-2 border-b border-red-500/20">
      <button
        onclick={() => selectedTab = 'overview'}
        class="px-6 py-3 text-sm font-medium transition-all relative {selectedTab === 'overview' ? 'text-red-100' : 'text-red-300/60 hover:text-red-300'}"
      >
        Vista General
        {#if selectedTab === 'overview'}
          <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-red-500 to-orange-500"></div>
        {/if}
      </button>
      <button
        onclick={() => selectedTab = 'detections'}
        class="px-6 py-3 text-sm font-medium transition-all relative {selectedTab === 'detections' ? 'text-red-100' : 'text-red-300/60 hover:text-red-300'}"
      >
        Detecciones ({analysis.detections.length})
        {#if selectedTab === 'detections'}
          <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-red-500 to-orange-500"></div>
        {/if}
      </button>
    </div>

    <!-- Tab Content -->
    {#if selectedTab === 'overview'}
      <div class="grid lg:grid-cols-2 gap-6">
        <!-- Configuration Card -->
        <Card variant="default" padding="lg">
          {#snippet header()}
            <h3 class="text-lg font-bold text-red-100 flex items-center gap-2">
              <svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              Configuración del Análisis
            </h3>
          {/snippet}

          <div class="space-y-4">
            <div class="flex justify-between items-center py-2 border-b border-red-500/10">
              <span class="text-red-300/70">Calidad de video</span>
              <span class="font-semibold text-red-100">{analysis.quality}</span>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-red-500/10">
              <span class="text-red-300/70">Frames por segundo</span>
              <span class="font-semibold text-red-100">{analysis.fps} FPS</span>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-red-500/10">
              <span class="text-red-300/70">Intervalo de análisis</span>
              <span class="font-semibold text-red-100">Cada {analysis.framesInterval} frames</span>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-red-500/10">
              <span class="text-red-300/70">Confianza mínima</span>
              <span class="font-semibold text-red-100">{(analysis.minConfidence * 100).toFixed(0)}%</span>
            </div>
            <div class="flex justify-between items-center py-2">
              <span class="text-red-300/70">Tiempo de procesamiento</span>
              <span class="font-semibold text-red-100">{analysis.processingTime}</span>
            </div>
          </div>
        </Card>

        <!-- Class Statistics -->
        <Card variant="default" padding="lg">
          {#snippet header()}
            <h3 class="text-lg font-bold text-red-100 flex items-center gap-2">
              <svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
              Estadísticas por Clase
            </h3>
          {/snippet}

          <div class="space-y-3 max-h-80 overflow-y-auto custom-scrollbar">
            {#each classStats as stat}
              <div class="bg-slate-800/40 rounded-lg p-3 border border-red-500/10">
                <div class="flex items-center justify-between mb-2">
                  <span class="font-medium text-red-100">{stat.class}</span>
                  <span class="text-sm text-red-300/70">{stat.count} detecciones</span>
                </div>
                <div class="flex items-center gap-2">
                  <div class="flex-1 h-2 bg-slate-900/60 rounded-full overflow-hidden">
                    <div 
                      class="h-full bg-gradient-to-r from-red-500 to-orange-500 transition-all"
                      style="width: {(stat.avgConfidence * 100).toFixed(0)}%"
                    ></div>
                  </div>
                  <span class="text-xs font-medium {getConfidenceColor(stat.avgConfidence)} min-w-[3rem] text-right">
                    {(stat.avgConfidence * 100).toFixed(0)}%
                  </span>
                </div>
              </div>
            {/each}
          </div>
        </Card>
      </div>
    {:else if selectedTab === 'detections'}
      <Card variant="default" padding="lg">
        <!-- Filter -->
        <div class="mb-6 flex items-center gap-3">
          <span class="text-sm text-red-300">Filtrar por clase:</span>
          <select
            bind:value={classFilter}
            class="px-4 py-2 bg-slate-900/60 border border-red-500/30 rounded-lg text-red-50 focus:border-red-500 focus:outline-none transition-all"
          >
            <option value="all">Todas las clases ({analysis.detections.length})</option>
            {#each analysis.detectedClasses as className}
              <option value={className}>
                {className} ({analysis.detections.filter(d => d.class === className).length})
              </option>
            {/each}
          </select>
        </div>

        <!-- Detections Table -->
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead class="bg-slate-900/50 border-y border-red-500/20">
              <tr>
                <th class="px-4 py-3 text-left text-xs font-semibold text-red-300 uppercase">Frame</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-red-300 uppercase">Tiempo</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-red-300 uppercase">Clase</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-red-300 uppercase">Confianza</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-red-300 uppercase">Posición</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-red-500/10">
              {#each filteredDetections as detection}
                <tr class="hover:bg-red-500/5 transition-colors">
                  <td class="px-4 py-3 text-sm text-red-100">#{detection.frameNumber}</td>
                  <td class="px-4 py-3 text-sm text-red-200">{detection.timestamp}</td>
                  <td class="px-4 py-3">
                    <span class="px-2 py-1 bg-slate-800/60 border border-red-500/20 rounded text-xs text-red-200">
                      {detection.class}
                    </span>
                  </td>
                  <td class="px-4 py-3">
                    <span class="text-sm font-medium {getConfidenceColor(detection.confidence)}">
                      {(detection.confidence * 100).toFixed(1)}%
                    </span>
                  </td>
                  <td class="px-4 py-3 text-xs text-red-300/70 font-mono">
                    x:{detection.bbox.x} y:{detection.bbox.y} w:{detection.bbox.width} h:{detection.bbox.height}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </Card>
    {/if}
  </div>
{/if}

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }

  .custom-scrollbar::-webkit-scrollbar-track {
    background: rgba(15, 23, 42, 0.5);
    border-radius: 3px;
  }

  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(239, 68, 68, 0.3);
    border-radius: 3px;
  }

  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(239, 68, 68, 0.5);
  }
</style>