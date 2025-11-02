<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import ConfirmationDialog from '$lib/components/ui/ConfirmationDialog.svelte';
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';

  // Clases COCO
  const COCO_CLASSES = [
    'person', 'bicycle', 'car', 'motorcycle', 'airplane', 'bus', 'train', 'truck', 'boat',
    'traffic light', 'fire hydrant', 'stop sign', 'parking meter', 'bench', 'bird', 'cat',
    'dog', 'horse', 'sheep', 'cow', 'elephant', 'bear', 'zebra', 'giraffe', 'backpack',
    'umbrella', 'handbag', 'tie', 'suitcase', 'frisbee', 'skis', 'snowboard', 'sports ball',
    'kite', 'baseball bat', 'baseball glove', 'skateboard', 'surfboard', 'tennis racket',
    'bottle', 'wine glass', 'cup', 'fork', 'knife', 'spoon', 'bowl', 'banana', 'apple',
    'sandwich', 'orange', 'broccoli', 'carrot', 'hot dog', 'pizza', 'donut', 'cake',
    'chair', 'couch', 'potted plant', 'bed', 'dining table', 'toilet', 'tv', 'laptop',
    'mouse', 'remote', 'keyboard', 'cell phone', 'microwave', 'oven', 'toaster', 'sink',
    'refrigerator', 'book', 'clock', 'vase', 'scissors', 'teddy bear', 'hair drier', 'toothbrush'
  ];

  interface Model {
    id: string;
    name: string;
    fileName: string;
    size: string;
    speed: string;
    precision: string;
    downloaded: boolean;
    active: boolean;
    isCustom: boolean;
    selectedClasses: Record<string, boolean>;
    downloading?: boolean;
    downloadProgress?: number;
  }

  // Modelos base de YOLO (mock data para diseño)
  let models = $state<Model[]>([
    {
      id: '1',
      name: 'YOLOv11n',
      fileName: 'yolo11n.pt',
      size: '2.6 MB',
      speed: 'Muy rápida',
      precision: '~39% mAP',
      downloaded: false,
      active: false,
      isCustom: false,
      selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
    },
    {
      id: '2',
      name: 'YOLOv11s',
      fileName: 'yolo11s.pt',
      size: '9.4 MB',
      speed: 'Rápida',
      precision: '~47% mAP',
      downloaded: true,
      active: true,
      isCustom: false,
      selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
    },
    {
      id: '3',
      name: 'YOLOv11m',
      fileName: 'yolo11m.pt',
      size: '20.1 MB',
      speed: 'Media',
      precision: '~51% mAP',
      downloaded: true,
      active: false,
      isCustom: false,
      selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
    },
    {
      id: '4',
      name: 'YOLOv11l',
      fileName: 'yolo11l.pt',
      size: '25.3 MB',
      speed: 'Lenta',
      precision: '~53% mAP',
      downloaded: false,
      active: false,
      isCustom: false,
      selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
    },
    {
      id: '5',
      name: 'YOLOv11x',
      fileName: 'yolo11x.pt',
      size: '56.9 MB',
      speed: 'Muy lenta',
      precision: '~54% mAP',
      downloaded: false,
      active: false,
      isCustom: false,
      selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
    },
    {
      id: '6',
      name: 'custom-model',
      fileName: 'custom-model.pt',
      size: '15.2 MB',
      speed: 'Desconocida',
      precision: 'Por determinar',
      downloaded: true,
      active: false,
      isCustom: true,
      selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
    }
  ]);

  let classModalOpen = $state(false);
  let currentModelForClasses = $state<Model | null>(null);
  let deleteDialogOpen = $state(false);
  let modelToDelete = $state<Model | null>(null);
  let isDragging = $state(false);
  let dragError = $state('');

  function toggleActive(modelId: string) {
    const model = models.find(m => m.id === modelId);
    if (model && model.downloaded) {
      model.active = !model.active;
    }
  }

  function openClassModal(model: Model) {
    currentModelForClasses = model;
    classModalOpen = true;
  }

  function toggleClass(className: string) {
    if (currentModelForClasses) {
      currentModelForClasses.selectedClasses[className] = !currentModelForClasses.selectedClasses[className];
    }
  }

  function toggleAllClasses() {
    if (currentModelForClasses) {
      const allSelected = Object.values(currentModelForClasses.selectedClasses).every(v => v);
      COCO_CLASSES.forEach(cls => {
        currentModelForClasses!.selectedClasses[cls] = !allSelected;
      });
    }
  }

  function getSelectedClassesCount(model: Model): number {
    return Object.values(model.selectedClasses).filter(v => v).length;
  }

  function downloadModel(modelId: string) {
    const model = models.find(m => m.id === modelId);
    if (model) {
      model.downloading = true;
      model.downloadProgress = 0;

      const interval = setInterval(() => {
        if (model.downloadProgress !== undefined && model.downloadProgress < 100) {
          model.downloadProgress += 5;
        } else {
          clearInterval(interval);
          model.downloading = false;
          model.downloaded = true;
          model.downloadProgress = undefined;
        }
      }, 200);
    }
  }

  function openDeleteDialog(model: Model) {
    modelToDelete = model;
    deleteDialogOpen = true;
  }

  function confirmDelete() {
    if (!modelToDelete) return;

    if (modelToDelete.isCustom) {
      models = models.filter(m => m.id !== modelToDelete.id);
    } else {
      modelToDelete.downloaded = false;
      modelToDelete.active = false;
    }
    
    modelToDelete = null;
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
    dragError = '';
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    dragError = '';

    const files = e.dataTransfer?.files;
    if (!files || files.length === 0) {
      dragError = 'No se detectaron archivos';
      return;
    }

    const file = files[0];
    
    if (!file.name.endsWith('.pt') && !file.name.endsWith('.onnx')) {
      dragError = 'Solo se aceptan archivos .pt o .onnx';
      return;
    }

    if (file.size > 100 * 1024 * 1024) {
      dragError = 'El archivo es demasiado grande (máx. 100MB)';
      return;
    }

    if (models.some(m => m.fileName === file.name)) {
      dragError = 'Ya existe un modelo con ese nombre';
      return;
    }

    const newModel: Model = {
      id: Date.now().toString(),
      name: file.name.replace(/\.(pt|onnx)$/, ''),
      fileName: file.name,
      size: formatBytes(file.size),
      speed: 'Desconocida',
      precision: 'Por determinar',
      downloaded: true,
      active: false,
      isCustom: true,
      selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
    };

    models = [...models, newModel];
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
</script>

<!-- Header -->
<div class="text-center mb-12">
  <h1 class="text-5xl font-bold bg-gradient-to-r from-red-400 via-orange-400 to-red-500 bg-clip-text text-transparent mb-4">
    Gestión de Modelos
  </h1>
  <p class="text-slate-300 text-lg">
    Administra y configura los modelos YOLO disponibles
  </p>
</div>

<div class="max-w-7xl mx-auto space-y-6">
  <!-- Models Table - Desktop View -->
  <Card variant="default" padding="none" class="hidden lg:block">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2 px-6 py-4">
        <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
        </svg>
        Modelos Disponibles
      </h2>
    {/snippet}

    <div class="overflow-x-auto">
      <table class="w-full">
        <thead class="bg-slate-900/50 border-y border-red-500/20">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-semibold text-red-300 uppercase tracking-wider">Estado</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-red-300 uppercase tracking-wider">Modelo</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-red-300 uppercase tracking-wider">Tamaño</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-red-300 uppercase tracking-wider">Velocidad</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-red-300 uppercase tracking-wider">Precisión</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-red-300 uppercase tracking-wider">Clases</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-red-300 uppercase tracking-wider">Acciones</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-red-500/10">
          {#each models as model (model.id)}
            <tr class="hover:bg-red-500/5 transition-colors">
              <td class="px-6 py-4">
                <input
                  type="checkbox"
                  checked={model.active}
                  disabled={!model.downloaded}
                  onchange={() => toggleActive(model.id)}
                  class="w-5 h-5 rounded border-red-500/30 text-red-500 focus:ring-red-500/50 bg-slate-800 cursor-pointer disabled:opacity-30 disabled:cursor-not-allowed"
                />
              </td>
              <td class="px-6 py-4">
                <div class="flex items-center gap-2">
                  <span class="text-sm font-semibold text-red-100">{model.name}</span>
                  {#if model.active}
                    <span class="px-2 py-0.5 text-xs font-medium bg-green-500/20 text-green-400 rounded-full border border-green-500/30">
                      Activo
                    </span>
                  {/if}
                  {#if model.isCustom}
                    <span class="px-2 py-0.5 text-xs font-medium bg-blue-500/20 text-blue-400 rounded-full border border-blue-500/30">
                      Personalizado
                    </span>
                  {/if}
                </div>
              </td>
              <td class="px-6 py-4">
                <span class="text-sm text-red-200">{model.size}</span>
              </td>
              <td class="px-6 py-4">
                <span class="text-sm text-red-200">{model.speed}</span>
              </td>
              <td class="px-6 py-4">
                <span class="text-sm text-red-200">{model.precision}</span>
              </td>
              <td class="px-6 py-4">
                <button
                  onclick={() => openClassModal(model)}
                  disabled={!model.downloaded}
                  class="px-3 py-1.5 bg-slate-900/60 border border-red-500/30 rounded-lg text-sm text-red-100 hover:border-red-500/50 transition-all flex items-center gap-2 disabled:opacity-30 disabled:cursor-not-allowed"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
                  </svg>
                  <span>{getSelectedClassesCount(model)}/{COCO_CLASSES.length}</span>
                </button>
              </td>
              <td class="px-6 py-4">
                {#if model.downloading}
                  <div class="w-32">
                    <div class="flex items-center gap-2 mb-1">
                      <span class="text-xs text-red-300">{model.downloadProgress}%</span>
                    </div>
                    <div class="w-full h-2 bg-slate-900/60 rounded-full overflow-hidden border border-red-500/30">
                      <div
                        class="h-full bg-gradient-to-r from-red-500 to-orange-500 transition-all duration-300"
                        style="width: {model.downloadProgress}%"
                      ></div>
                    </div>
                  </div>
                {:else if model.downloaded}
                  <Button
                    variant="danger"
                    size="sm"
                    onclick={() => openDeleteDialog(model)}
                  >
                    {#snippet icon()}
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    {/snippet}
                    Eliminar
                  </Button>
                {:else}
                  <Button
                    variant="primary"
                    size="sm"
                    onclick={() => downloadModel(model.id)}
                  >
                    {#snippet icon()}
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                      </svg>
                    {/snippet}
                    Descargar
                  </Button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </Card>

  <!-- Models Cards - Mobile/Tablet View -->
  <div class="lg:hidden space-y-4">
    <div class="flex items-center justify-between px-4">
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
        </svg>
        Modelos Disponibles
      </h2>
    </div>

    {#each models as model (model.id)}
      <Card variant="default" padding="lg">
        <div class="space-y-4">
          <!-- Header con nombre y checkbox -->
          <div class="flex items-start justify-between gap-3">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 mb-2">
                <h3 class="text-lg font-bold text-red-100 truncate">{model.name}</h3>
                <input
                  type="checkbox"
                  checked={model.active}
                  disabled={!model.downloaded}
                  onchange={() => toggleActive(model.id)}
                  class="w-5 h-5 rounded border-red-500/30 text-red-500 focus:ring-red-500/50 bg-slate-800 cursor-pointer disabled:opacity-30 disabled:cursor-not-allowed flex-shrink-0"
                />
              </div>
              <div class="flex flex-wrap items-center gap-2">
                {#if model.active}
                  <span class="px-2 py-0.5 text-xs font-medium bg-green-500/20 text-green-400 rounded-full border border-green-500/30">
                    Activo
                  </span>
                {/if}
                {#if model.isCustom}
                  <span class="px-2 py-0.5 text-xs font-medium bg-blue-500/20 text-blue-400 rounded-full border border-blue-500/30">
                    Personalizado
                  </span>
                {/if}
              </div>
            </div>
          </div>

          <!-- Info Grid -->
          <div class="grid grid-cols-2 gap-3 text-sm">
            <div>
              <span class="text-red-300/70 text-xs">Tamaño</span>
              <p class="text-red-100 font-medium">{model.size}</p>
            </div>
            <div>
              <span class="text-red-300/70 text-xs">Velocidad</span>
              <p class="text-red-100 font-medium">{model.speed}</p>
            </div>
            <div>
              <span class="text-red-300/70 text-xs">Precisión</span>
              <p class="text-red-100 font-medium">{model.precision}</p>
            </div>
            <div>
              <span class="text-red-300/70 text-xs">Clases</span>
              <p class="text-red-100 font-medium">{getSelectedClassesCount(model)}/{COCO_CLASSES.length}</p>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex flex-col sm:flex-row gap-2 pt-2">
            {#if model.downloading}
              <div class="w-full">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-xs text-red-300">Descargando...</span>
                  <span class="text-xs text-red-300 font-medium">{model.downloadProgress}%</span>
                </div>
                <div class="w-full h-2 bg-slate-900/60 rounded-full overflow-hidden border border-red-500/30">
                  <div
                    class="h-full bg-gradient-to-r from-red-500 to-orange-500 transition-all duration-300"
                    style="width: {model.downloadProgress}%"
                  ></div>
                </div>
              </div>
            {:else if model.downloaded}
              <button
                onclick={() => openClassModal(model)}
                class="flex-1 px-4 py-2.5 bg-slate-900/60 border border-red-500/30 rounded-lg text-sm text-red-100 hover:border-red-500/50 transition-all flex items-center justify-center gap-2"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
                </svg>
                Configurar Clases
              </button>
              <Button
                variant="danger"
                size="md"
                onclick={() => openDeleteDialog(model)}
                class="sm:w-auto"
              >
                {#snippet icon()}
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                {/snippet}
                Eliminar
              </Button>
            {:else}
              <Button
                variant="primary"
                size="md"
                onclick={() => downloadModel(model.id)}
                class="w-full"
              >
                {#snippet icon()}
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                  </svg>
                {/snippet}
                Descargar Modelo
              </Button>
            {/if}
          </div>
        </div>
      </Card>
    {/each}
  </div>

  <!-- Drag & Drop Area -->
  <Card variant="default" padding="lg">
    {#snippet header()}
      <h2 class="text-xl font-bold text-red-100 flex items-center gap-2">
        <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
        </svg>
        Agregar Modelo Personalizado
      </h2>
    {/snippet}

    <div
      class="border-2 border-dashed rounded-xl p-12 transition-all duration-300 {isDragging ? 'border-red-500 bg-red-500/10' : 'border-red-500/30 hover:border-red-500/50 hover:bg-red-500/5'}"
      ondragover={handleDragOver}
      ondragleave={handleDragLeave}
      ondrop={handleDrop}
    >
      <div class="flex flex-col items-center justify-center gap-4 text-center">
        <div class="w-16 h-16 rounded-full bg-red-500/20 flex items-center justify-center">
          <svg class="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
          </svg>
        </div>
        
        <div>
          <h3 class="text-lg font-semibold text-red-100 mb-2">
            {isDragging ? 'Suelta el archivo aquí' : 'Arrastra y suelta tu modelo'}
          </h3>
          <p class="text-sm text-red-300/70">
            Formatos aceptados: .pt, .onnx, .engine (máx. 100MB)
          </p>
        </div>

        {#if dragError}
          <ErrorMessage
            variant="error"
            message={dragError}
            dismissible={true}
            onDismiss={() => dragError = ''}
            class="max-w-md"
          />
        {/if}
      </div>
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
        <p>Activa los modelos que desees usar para el análisis de video</p>
      </div>
      <div class="flex items-start gap-2">
        <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p>Los modelos más pequeños son más rápidos pero menos precisos</p>
      </div>
      <div class="flex items-start gap-2">
        <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p>Configura las clases específicas que cada modelo debe detectar</p>
      </div>
      <div class="flex items-start gap-2">
        <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p>Puedes agregar modelos personalizados arrastrándolos al área indicada</p>
      </div>
    </div>
  </Card>
</div>

<!-- Class Selection Modal -->
{#if classModalOpen && currentModelForClasses}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <!-- Backdrop -->
    <button
      class="absolute inset-0 bg-slate-950/80 backdrop-blur-sm"
      onclick={() => classModalOpen = false}
      aria-label="Cerrar modal"
    ></button>

    <!-- Modal -->
    <div class="relative bg-slate-900 border-2 border-red-500/50 rounded-xl shadow-2xl shadow-red-500/20 w-full max-w-3xl max-h-[80vh] flex flex-col">
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-red-500/20">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-lg bg-red-500/20 flex items-center justify-center">
            <svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
            </svg>
          </div>
          <div>
            <h3 class="text-xl font-bold text-red-100">Clases a Detectar</h3>
            <p class="text-sm text-red-300/70">{currentModelForClasses.name}</p>
          </div>
        </div>
        <button
          onclick={() => classModalOpen = false}
          class="w-10 h-10 rounded-lg hover:bg-red-500/10 transition-colors flex items-center justify-center text-red-300 hover:text-red-100"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto overflow-x-hidden custom-scrollbar">
        <div class="sticky top-0 bg-slate-900 px-6 pt-4 pb-3 border-b border-red-500/20 z-10">
          <div class="flex items-center justify-between gap-4">
            <div class="text-sm text-red-300">
              <span class="font-semibold text-red-100">{getSelectedClassesCount(currentModelForClasses)}</span> de <span class="font-medium">{COCO_CLASSES.length}</span> seleccionadas
            </div>
            <Button variant="ghost" size="sm" onclick={toggleAllClasses}>
              {Object.values(currentModelForClasses.selectedClasses).every(v => v) ? 'Deseleccionar Todo' : 'Seleccionar Todo'}
            </Button>
          </div>
        </div>

        <div class="px-6 py-4">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          {#each COCO_CLASSES as className}
            <label class="flex items-center gap-3 p-3 bg-slate-800/40 border border-red-500/20 rounded-lg hover:border-red-500/40 hover:bg-slate-800/60 transition-all cursor-pointer group">
              <input
                type="checkbox"
                checked={currentModelForClasses.selectedClasses[className]}
                onchange={() => toggleClass(className)}
                class="w-5 h-5 rounded border-red-500/30 text-red-500 focus:ring-red-500/50 bg-slate-800 cursor-pointer flex-shrink-0"
              />
              <span class="flex-1 text-sm font-medium {currentModelForClasses.selectedClasses[className] ? 'text-red-100' : 'text-red-300/50'} group-hover:text-red-100 transition-colors">
                {className}
              </span>
            </label>
          {/each}
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex items-center justify-end gap-3 p-6 border-t border-red-500/20 bg-slate-900/50">
        <Button variant="primary" onclick={() => classModalOpen = false}>
          {#snippet icon()}
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          {/snippet}
          Guardar
        </Button>
      </div>
    </div>
  </div>
{/if}

<!-- Delete Confirmation Dialog -->
<ConfirmationDialog
  bind:open={deleteDialogOpen}
  variant="danger"
  title="¿Eliminar modelo?"
  message="Esta acción eliminará el modelo {modelToDelete?.name} de tu sistema. {modelToDelete?.isCustom ? 'El modelo personalizado se eliminará permanentemente.' : 'Podrás descargarlo nuevamente si lo necesitas en el futuro.'}"
  confirmText="Eliminar"
  cancelText="Cancelar"
  onConfirm={confirmDelete}
/>

<style>
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