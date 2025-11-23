import { invoke } from '@tauri-apps/api/core';

export const COCO_CLASSES = [
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

export interface Model {
  id: string;
  name: string;
  fileName: string;
  downloadUrl: string;
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

export const YOLO_MODELS_BASE_URL = 'https://github.com/ultralytics/assets/releases/download/v8.3.0';

export const DEFAULT_MODELS: Model[] = [
  {
    id: '1',
    name: 'YOLOv11n',
    fileName: 'yolo11n.pt',
    downloadUrl: `${YOLO_MODELS_BASE_URL}/yolo11n.pt`,
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
    downloadUrl: `${YOLO_MODELS_BASE_URL}/yolo11s.pt`,
    size: '9.4 MB',
    speed: 'Rápida',
    precision: '~47% mAP',
    downloaded: false,
    active: false,
    isCustom: false,
    selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
  },
  {
    id: '3',
    name: 'YOLOv11m',
    fileName: 'yolo11m.pt',
    downloadUrl: `${YOLO_MODELS_BASE_URL}/yolo11m.pt`,
    size: '20.1 MB',
    speed: 'Media',
    precision: '~51% mAP',
    downloaded: false,
    active: false,
    isCustom: false,
    selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
  },
  {
    id: '4',
    name: 'YOLOv11l',
    fileName: 'yolo11l.pt',
    downloadUrl: `${YOLO_MODELS_BASE_URL}/yolo11l.pt`,
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
    downloadUrl: `${YOLO_MODELS_BASE_URL}/yolo11x.pt`,
    size: '56.9 MB',
    speed: 'Muy lenta',
    precision: '~54% mAP',
    downloaded: false,
    active: false,
    isCustom: false,
    selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
  }
];

export async function getDownloadedModels(): Promise<string[]> {
  try {
    return await invoke<string[]>('list_downloaded_models');
  } catch (error) {
    console.error('Error listing downloaded models:', error);
    return [];
  }
}

export function mergeModelsWithDownloads(knownModels: Model[], downloadedFiles: string[]): Model[] {
  // Mark known models as downloaded
  const updatedModels = knownModels.map(m => ({
    ...m,
    downloaded: downloadedFiles.includes(m.fileName)
  }));

  // Add unknown (custom) models
  const knownFileNames = new Set(knownModels.map(m => m.fileName));
  const customFiles = downloadedFiles.filter(f => !knownFileNames.has(f));

  const customModels: Model[] = customFiles.map(f => ({
    id: `custom-${f}`,
    name: f.replace(/\.(pt|onnx)$/, ''),
    fileName: f,
    downloadUrl: '',
    size: 'Desconocido',
    speed: 'Desconocida',
    precision: 'Desconocida',
    downloaded: true,
    active: false,
    isCustom: true,
    selectedClasses: COCO_CLASSES.reduce((acc, cls) => ({ ...acc, [cls]: true }), {})
  }));

  return [...updatedModels, ...customModels];
}
