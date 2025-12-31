import { writable, derived } from 'svelte/store';
import { loadAppConfig } from '$lib/config';
import type { Translations } from './translations/index.js';
import { es } from './translations/es.js';
import { en } from './translations/en.js';
import { eu } from './translations/eu.js';

// Available translations
const translations: Record<string, Translations> = {
    es,
    en,
    eu
};

// Create a writable store for the current language
function createLanguageStore() {
    const { subscribe, set } = writable<string>('es');

    return {
        subscribe,
        set: (lang: string) => {
            if (translations[lang]) {
                set(lang);
            }
        },
        init: () => {
            const config = loadAppConfig();
            set(config.language || 'es');
        }
    };
}

export const currentLanguage = createLanguageStore();

// Derived store that provides the current translations
export const t = derived(currentLanguage, ($lang) => translations[$lang] || translations.es);

// Helper function to get a translation
export function translate(key: string): string {
    let result: any = translations;
    const lang = loadAppConfig().language || 'es';

    result = translations[lang] || translations.es;

    const keys = key.split('.');
    for (const k of keys) {
        if (result && typeof result === 'object' && k in result) {
            result = result[k];
        } else {
            return key; // Return the key if translation not found
        }
    }

    return typeof result === 'string' ? result : key;
}
