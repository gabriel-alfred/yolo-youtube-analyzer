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
