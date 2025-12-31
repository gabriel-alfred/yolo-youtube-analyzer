import type { Model } from './models';

const STORAGE_KEY = 'yolo_model_config';

export interface ModelConfig {
    id: string;
    active: boolean;
    selectedClasses: Record<string, boolean>;
}

export function saveModelsConfig(models: Model[]) {
    const config: ModelConfig[] = models.map(m => ({
        id: m.id,
        active: m.active,
        selectedClasses: m.selectedClasses
    }));
    localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
}

export function loadModelsConfig(): ModelConfig[] | null {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) return null;
    try {
        return JSON.parse(stored);
    } catch (e) {
        console.error("Failed to parse model config", e);
        return null;
    }
}

export function applyConfigToModels(models: Model[], config: ModelConfig[]): Model[] {
    return models.map(m => {
        const saved = config.find(c => c.id === m.id);
        if (saved) {
            return {
                ...m,
                active: saved.active,
                selectedClasses: { ...m.selectedClasses, ...saved.selectedClasses }
            };
        }
        return m;
    });
}

// Global App Config
const APP_CONFIG_KEY = 'yolo_app_config';

export interface AppConfig {
    processingDevice: 'cpu' | 'cuda' | 'mps';
    language: string;
    compressResults: boolean;
    maxStorageSize: number;
}

export const DEFAULT_APP_CONFIG: AppConfig = {
    processingDevice: 'cpu',
    language: 'es',
    compressResults: true,
    maxStorageSize: 50
};

export function saveAppConfig(config: AppConfig) {
    localStorage.setItem(APP_CONFIG_KEY, JSON.stringify(config));
}

export function loadAppConfig(): AppConfig {
    const stored = localStorage.getItem(APP_CONFIG_KEY);
    if (!stored) return DEFAULT_APP_CONFIG;
    try {
        const parsed = JSON.parse(stored);
        return { ...DEFAULT_APP_CONFIG, ...parsed }; // Merge with defaults to ensure all fields exist
    } catch (e) {
        console.error("Failed to parse app config", e);
        return DEFAULT_APP_CONFIG;
    }
}
