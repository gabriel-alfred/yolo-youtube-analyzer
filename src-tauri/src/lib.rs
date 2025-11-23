use tauri::Manager;
use tauri::Emitter;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadProgress {
    model_id: String,
    downloaded: u64,
    total: u64,
    percentage: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    models_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            models_path: None,
        }
    }
}

fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map(|p| p.join("config.json"))
        .map_err(|e| e.to_string())
}

fn load_config(app: &tauri::AppHandle) -> AppConfig {
    if let Ok(config_path) = get_config_path(app) {
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
    }
    AppConfig::default()
}

fn save_config(app: &tauri::AppHandle, config: &AppConfig) -> Result<(), String> {
    let config_path = get_config_path(app)?;
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_path, content).map_err(|e| e.to_string())?;
    Ok(())
}

fn resolve_models_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let config = load_config(app);
    
    if let Some(path_str) = config.models_path {
        let path = PathBuf::from(path_str);
        if !path.exists() {
             fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        }
        return Ok(path);
    }

    // Default to Documents/YoloModels if available, otherwise AppData/models
    let default_path = if let Ok(docs_dir) = app.path().document_dir() {
        docs_dir.join("YoloModels")
    } else {
        app.path()
            .app_data_dir()
            .map_err(|e| e.to_string())?
            .join("models")
    };

    if !default_path.exists() {
        fs::create_dir_all(&default_path).map_err(|e| e.to_string())?;
    }

    Ok(default_path)
}

#[tauri::command]
fn get_current_models_dir(app: tauri::AppHandle) -> Result<String, String> {
    let path = resolve_models_dir(&app)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn set_models_dir(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let mut config = load_config(&app);
    config.models_path = Some(path);
    save_config(&app, &config)?;
    Ok(())
}

// Comando para obtener el directorio de modelos (usado internamente o por frontend)
#[tauri::command]
fn get_models_dir(app: tauri::AppHandle) -> Result<String, String> {
    let models_dir = resolve_models_dir(&app)?;
    Ok(models_dir.to_string_lossy().to_string())
}

// Comando para listar modelos descargados
#[tauri::command]
fn list_downloaded_models(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let models_dir = resolve_models_dir(&app)?;
    
    let mut models = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&models_dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if file_name.ends_with(".pt") || file_name.ends_with(".onnx") {
                    models.push(file_name);
                }
            }
        }
    }
    
    Ok(models)
}

// Comando para verificar si un modelo existe
#[tauri::command]
fn check_model_exists(app: tauri::AppHandle, file_name: String) -> Result<bool, String> {
    let models_dir = resolve_models_dir(&app)?;
    let model_path = models_dir.join(&file_name);
    Ok(model_path.exists())
}

// Comando para descargar un modelo
#[tauri::command]
async fn download_model(
    app: tauri::AppHandle,
    window: tauri::Window,
    model_id: String,
    file_name: String,
    url: String,
) -> Result<String, String> {
    let models_dir = resolve_models_dir(&app)?;
    let file_path = models_dir.join(&file_name);
    
    // Usar reqwest para descargar
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600)) // 10 minutos
        .build()
        .map_err(|e| e.to_string())?;
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Error al iniciar descarga: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Error HTTP: {}", response.status()));
    }
    
    let total_size = response.content_length().unwrap_or(0);
    
    let mut file = fs::File::create(&file_path)
        .map_err(|e| format!("Error al crear archivo: {}", e))?;
    
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    
    use futures_util::StreamExt;
    use std::io::Write;
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| format!("Error descargando: {}", e))?;
        
        file.write_all(&chunk)
            .map_err(|e| format!("Error escribiendo: {}", e))?;
        
        downloaded += chunk.len() as u64;
        
        let percentage = if total_size > 0 {
            (downloaded as f32 / total_size as f32) * 100.0
        } else {
            0.0
        };
        
        // Emitir progreso
        let progress = DownloadProgress {
            model_id: model_id.clone(),
            downloaded,
            total: total_size,
            percentage,
        };
        
        window.emit("download-progress", &progress).ok();
    }
    
    Ok(file_path.to_string_lossy().to_string())
}

// Comando para eliminar un modelo
#[tauri::command]
fn delete_model(app: tauri::AppHandle, file_name: String) -> Result<(), String> {
    let models_dir = resolve_models_dir(&app)?;
    let file_path = models_dir.join(&file_name);
    
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

// Comando para obtener el tamaño de un archivo
#[tauri::command]
fn get_file_size(app: tauri::AppHandle, file_name: String) -> Result<u64, String> {
    let models_dir = resolve_models_dir(&app)?;
    let file_path = models_dir.join(&file_name);
    
    if file_path.exists() {
        let metadata = fs::metadata(&file_path).map_err(|e| e.to_string())?;
        Ok(metadata.len())
    } else {
        Err("Archivo no encontrado".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_models_dir,
            get_current_models_dir,
            set_models_dir,
            list_downloaded_models,
            check_model_exists,
            download_model,
            delete_model,
            get_file_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}