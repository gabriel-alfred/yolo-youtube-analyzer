use tauri::Manager;
use tauri::Emitter;
use tauri_plugin_shell::ShellExt;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use std::process::Child;

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

// Estructura para mantener el estado de la sesión de análisis
pub struct AnalysisSession {
    id: String,
    #[allow(dead_code)] // Se usa para matar el proceso
    process: Child,
    metadata: AnalysisConfig,
}

// Configuración del análisis para guardar en estado
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisConfig {
    url: String,
    model_name: String,
    conf: f32,
    classes: Option<String>,
    device: String,
    frames: i32,
    quality: i32,
}

// Estado global de la aplicación
pub struct AppState {
    active_session: Mutex<Option<AnalysisSession>>,
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

// Función para encontrar Python en el sistema
fn find_python_executable() -> Result<String, String> {
    // En Windows, intenta estas ubicaciones comunes
    if cfg!(windows) {
        let common_paths = vec![
            r"C:\Python310\python.exe",
            r"C:\Python311\python.exe",
            r"C:\Python312\python.exe",
            r"C:\Program Files\Python310\python.exe",
            r"C:\Program Files\Python311\python.exe",
            r"C:\Program Files\Python312\python.exe",
            r"C:\Users\*\AppData\Local\Programs\Python\Python310\python.exe",
            r"C:\Users\*\AppData\Local\Programs\Python\Python311\python.exe",
            r"C:\Users\*\AppData\Local\Programs\Python\Python312\python.exe",
        ];
        
        for path_str in common_paths {
            // Expande * con el username si es necesario
            let path = if path_str.contains("*") {
                if let Ok(output) = std::process::Command::new("whoami")
                    .output() {
                    let username = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    PathBuf::from(path_str.replace("*", &username))
                } else {
                    PathBuf::from(path_str)
                }
            } else {
                PathBuf::from(path_str)
            };
            
            if path.exists() {
                return Ok(path.to_string_lossy().to_string());
            }
        }
    }
    
    // Intenta usar py launcher (Windows)
    if cfg!(windows) {
        if let Ok(output) = std::process::Command::new("py")
            .arg("-c")
            .arg("import sys; print(sys.executable)")
            .output() 
        {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() && PathBuf::from(&path).exists() {
                    return Ok(path);
                }
            }
        }
    }
    
    // Intenta which/where en PATH
    let python_names = if cfg!(windows) {
        vec!["python.exe", "python3.exe", "python"]
    } else {
        vec!["python3", "python"]
    };

    for python_name in python_names {
        if let Ok(output) = std::process::Command::new(if cfg!(windows) { "where" } else { "which" })
            .arg(python_name)
            .output()
        {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().lines().next().unwrap_or("").to_string();
                if !path.is_empty() {
                    return Ok(path);
                }
            }
        }
    }

    Err("Python no encontrado en el sistema".to_string())
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

#[tauri::command]
fn get_models_dir(app: tauri::AppHandle) -> Result<String, String> {
    let models_dir = resolve_models_dir(&app)?;
    Ok(models_dir.to_string_lossy().to_string())
}

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

#[tauri::command]
fn check_model_exists(app: tauri::AppHandle, file_name: String) -> Result<bool, String> {
    let models_dir = resolve_models_dir(&app)?;
    let model_path = models_dir.join(&file_name);
    Ok(model_path.exists())
}

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
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600))
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

#[tauri::command]
fn delete_model(app: tauri::AppHandle, file_name: String) -> Result<(), String> {
    let models_dir = resolve_models_dir(&app)?;
    let file_path = models_dir.join(&file_name);
    
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

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

#[tauri::command]
fn import_model(app: tauri::AppHandle, file_path: String) -> Result<String, String> {
    let models_dir = resolve_models_dir(&app)?;
    let source_path = PathBuf::from(&file_path);
    
    if !source_path.exists() {
        return Err("El archivo de origen no existe".to_string());
    }
    
    let file_name = source_path.file_name()
        .ok_or("Nombre de archivo inválido")?
        .to_string_lossy()
        .to_string();
        
    let dest_path = models_dir.join(&file_name);
    
    fs::copy(&source_path, &dest_path).map_err(|e| e.to_string())?;
    
    Ok(file_name)
}

#[tauri::command]
async fn start_live_analysis(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    url: String,
    model_name: String,
    conf: f32,
    classes: Option<String>,
    device: String,
    quality: String,
) -> Result<String, String> {
    // Verificar si ya hay un análisis en curso
    {
        let mut active_session = state.active_session.lock().map_err(|e| e.to_string())?;
        if active_session.is_some() {
            return Err("Ya hay un análisis en curso. Detenlo antes de iniciar uno nuevo.".to_string());
        }
    }

    let models_dir = resolve_models_dir(&app)?;
    let model_path = models_dir.join(&model_name);
    
    if !model_path.exists() {
        return Err("Model file not found".to_string());
    }

    let resource_path = app.path().resolve("python/analyze_live.py", tauri::path::BaseDirectory::Resource)
        .unwrap_or_else(|_| PathBuf::from(""));

    let script_path = if resource_path.exists() {
        resource_path
    } else {
        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."));
        
        let candidates = vec![
            exe_path.parent().unwrap().join("../src-tauri/python/analyze_live.py"),
            PathBuf::from("src-tauri/python/analyze_live.py"),
            PathBuf::from("./python/analyze_live.py"),
        ];
        
        let mut found = false;
        let mut script = PathBuf::new();
        
        for candidate in candidates {
            if candidate.exists() {
                found = true;
                script = candidate;
                break;
            }
        }
        
        if !found {
            return Err(format!("Script analyze_live.py no encontrado"));
        }
        
        script
    };

    let python_exe = find_python_executable()?;

    let mut args = vec![
        script_path.to_string_lossy().to_string(),
        "--url".to_string(), url.clone(),
        "--model".to_string(), model_path.to_string_lossy().to_string(),
        "--conf".to_string(), conf.to_string(),
        "--device".to_string(), device.clone(),
        "--quality".to_string(), quality.clone(),
    ];

    if let Some(cls) = &classes {
        args.push("--classes".to_string());
        args.push(cls.clone());
    }

    let mut command = std::process::Command::new(&python_exe);
    command.args(&args);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to spawn python: {} (path: {})", e, python_exe))?;

    let stdout = child.stdout.take()
        .ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take()
        .ok_or("Failed to capture stderr")?;

    let app_handle = app.clone();
    let app_handle_stderr = app.clone();

    // Thread para leer stdout en tiempo real
    std::thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                        let _ = app_handle.emit("analysis-progress", &json_val);
                    } else {
                        eprintln!("Python stdout: {}", line);
                    }
                }
            }
        }
    });

    // Thread para leer stderr en tiempo real
    std::thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(stderr);
        
        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    eprintln!("Python stderr: {}", line);
                    let _ = app_handle_stderr.emit("analysis-error", line.to_string());
                }
            }
        }
    });

    let session_id = Uuid::new_v4().to_string();
    
    // Reuse AnalysisConfig but with dummy values for unused fields
    let config = AnalysisConfig {
        url,
        model_name,
        conf,
        classes,
        device,
        frames: 1,
        quality: 0, // Not used for live
    };

    let session = AnalysisSession {
        id: session_id.clone(),
        process: child,
        metadata: config,
    };

    {
        let mut active_session = state.active_session.lock().map_err(|e| e.to_string())?;
        *active_session = Some(session);
    }

    Ok(session_id)
}

#[tauri::command]
async fn start_video_analysis(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    url: String,
    model_name: String,
    conf: f32,
    classes: Option<String>,
    device: String,
    frames: i32,
    quality: i32,
) -> Result<String, String> {
    // Verificar si ya hay un análisis en curso
    {
        let mut active_session = state.active_session.lock().map_err(|e| e.to_string())?;
        if active_session.is_some() {
            return Err("Ya hay un análisis en curso. Detenlo antes de iniciar uno nuevo.".to_string());
        }
    }

    let models_dir = resolve_models_dir(&app)?;
    let model_path = models_dir.join(&model_name);
    
    if !model_path.exists() {
        return Err("Model file not found".to_string());
    }

    let output_dir = app.path().download_dir()
        .map_err(|e| e.to_string())?
        .join("YoloAnalysis");

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    }

    let resource_path = app.path().resolve("python/analyze_video.py", tauri::path::BaseDirectory::Resource)
        .unwrap_or_else(|_| PathBuf::from(""));

    let script_path = if resource_path.exists() {
        resource_path
    } else {
        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."));
        
        let candidates = vec![
            exe_path.parent().unwrap().join("../src-tauri/python/analyze_video.py"),
            PathBuf::from("src-tauri/python/analyze_video.py"),
            PathBuf::from("./python/analyze_video.py"),
        ];
        
        let mut found = false;
        let mut script = PathBuf::new();
        
        for candidate in candidates {
            if candidate.exists() {
                found = true;
                script = candidate;
                break;
            }
        }
        
        if !found {
            return Err(format!("Script no encontrado en ninguna ubicación esperada"));
        }
        
        script
    };

    let python_exe = find_python_executable()?;

    let mut args = vec![
        script_path.to_string_lossy().to_string(),
        "--url".to_string(), url.clone(),
        "--model".to_string(), model_path.to_string_lossy().to_string(),
        "--output_dir".to_string(), output_dir.to_string_lossy().to_string(),
        "--conf".to_string(), conf.to_string(),
        "--device".to_string(), device.clone(),
        "--frames".to_string(), frames.to_string(),
        "--quality".to_string(), quality.to_string(),
    ];

    if let Some(cls) = &classes {
        args.push("--classes".to_string());
        args.push(cls.clone());
    }

    let mut command = std::process::Command::new(&python_exe);
    command.args(&args);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to spawn python: {} (path: {})", e, python_exe))?;

    let stdout = child.stdout.take()
        .ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take()
        .ok_or("Failed to capture stderr")?;

    let app_handle = app.clone();
    let app_handle_stderr = app.clone();

    // Thread para leer stdout en tiempo real
    std::thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                        let _ = app_handle.emit("analysis-progress", &json_val);
                        
                        // Si el análisis se completa, limpiar la sesión
                        if let Some(status) = json_val.get("status").and_then(|s| s.as_str()) {
                            if status == "complete" || status == "error" {
                                // Nota: No podemos limpiar la sesión aquí fácilmente porque necesitamos el State
                                // Pero el frontend puede llamar a un comando para reconocer que terminó
                            }
                        }
                    } else {
                        eprintln!("Python stdout: {}", line);
                    }
                }
            }
        }
    });

    // Thread para leer stderr en tiempo real
    std::thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(stderr);
        
        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !line.contains("DEBUG") && !line.contains("WARNING") {
                    eprintln!("Python stderr: {}", line);
                    let _ = app_handle_stderr.emit("analysis-error", line.to_string());
                }
            }
        }
    });

    let session_id = Uuid::new_v4().to_string();
    
    let config = AnalysisConfig {
        url,
        model_name,
        conf,
        classes,
        device,
        frames,
        quality,
    };

    let session = AnalysisSession {
        id: session_id.clone(),
        process: child,
        metadata: config,
    };

    {
        let mut active_session = state.active_session.lock().map_err(|e| e.to_string())?;
        *active_session = Some(session);
    }

    Ok(session_id)
}

#[tauri::command]
fn stop_video_analysis(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut active_session = state.active_session.lock().map_err(|e| e.to_string())?;
    
    if let Some(mut session) = active_session.take() {
        // Intentar matar el proceso
        let _ = session.process.kill();
        // Esperar a que termine para evitar zombies
        let _ = session.process.wait();
        return Ok(());
    }
    
    Err("No hay análisis activo para detener".to_string())
}

#[derive(Serialize)]
struct ActiveAnalysisInfo {
    id: String,
    config: AnalysisConfig,
}

#[tauri::command]
fn get_active_analysis(state: tauri::State<'_, AppState>) -> Result<Option<ActiveAnalysisInfo>, String> {
    let mut active_session = state.active_session.lock().map_err(|e| e.to_string())?;
    
    let mut should_clear = false;
    
    if let Some(session) = &mut *active_session {
        match session.process.try_wait() {
            Ok(Some(_)) => {
                // Process has exited
                should_clear = true;
            }
            Ok(None) => {
                // Process is still running
                return Ok(Some(ActiveAnalysisInfo {
                    id: session.id.clone(),
                    config: session.metadata.clone(),
                }));
            }
            Err(_) => {
                // Error checking status, assume dead
                should_clear = true;
            }
        }
    }
    
    if should_clear {
        *active_session = None;
    }
    
    Ok(None)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Detection {
    frame_number: i32,
    timestamp: String,
    class: String,
    confidence: f32,
    bbox: BBox,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResult {
    id: String,
    video_url: String,
    thumbnail: String,
    title: String,
    duration: String,
    analyzed_date: String,
    total_objects: i32,
    detected_classes: Vec<String>,
    model: String,
    quality: String,
    frames_interval: i32,
    min_confidence: f32,
    processing_time: String,
    result_path: String,
    fps: i32,
    detections: Vec<Detection>,
    #[serde(default)]
    status: Option<String>,
}

#[tauri::command]
fn get_all_analysis_results(app: tauri::AppHandle) -> Result<Vec<AnalysisResult>, String> {
    let output_dir = app.path().download_dir()
        .map_err(|e| e.to_string())?
        .join("YoloAnalysis");

    if !output_dir.exists() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(output_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(result) = serde_json::from_str::<AnalysisResult>(&content) {
                        results.push(result);
                    } else {
                        // Try legacy format
                        #[derive(Deserialize)]
                        #[serde(rename_all = "camelCase")]
                        struct LegacyAnalysisResult {
                            id: String,
                            video_url: String,
                            thumbnail: String,
                            title: String,
                            duration: String,
                            analyzed_date: String,
                            total_objects: i32,
                            detected_classes: Vec<String>,
                            model: String,
                            quality: String,
                            frames_interval: i32,
                            min_confidence: f32,
                            processing_time: String,
                            result_path: String,
                        }

                        if let Ok(legacy) = serde_json::from_str::<LegacyAnalysisResult>(&content) {
                            results.push(AnalysisResult {
                                id: legacy.id,
                                video_url: legacy.video_url,
                                thumbnail: legacy.thumbnail,
                                title: legacy.title,
                                duration: legacy.duration,
                                analyzed_date: legacy.analyzed_date,
                                total_objects: legacy.total_objects,
                                detected_classes: legacy.detected_classes,
                                model: legacy.model,
                                quality: legacy.quality,
                                frames_interval: legacy.frames_interval,
                                min_confidence: legacy.min_confidence,
                                processing_time: legacy.processing_time,
                                result_path: legacy.result_path,
                                fps: 30, // Default
                                detections: Vec::new(), // Default
                                status: Some("complete".to_string()),
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort by date descending
    results.sort_by(|a, b| b.analyzed_date.cmp(&a.analyzed_date));

    Ok(results)
}

#[tauri::command]
fn get_analysis_result(app: tauri::AppHandle, id: String) -> Result<AnalysisResult, String> {
    let output_dir = app.path().download_dir()
        .map_err(|e| e.to_string())?
        .join("YoloAnalysis");

    if !output_dir.exists() {
        return Err("Analysis directory not found".to_string());
    }

    // The Python script saves files as analyzed_{video_id}.json
    let file_path = output_dir.join(format!("analyzed_{}.json", id));
    
    if file_path.exists() {
        let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
        if let Ok(result) = serde_json::from_str::<AnalysisResult>(&content) {
            return Ok(result);
        }
        
        // Try legacy format
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct LegacyAnalysisResult {
            id: String,
            video_url: String,
            thumbnail: String,
            title: String,
            duration: String,
            analyzed_date: String,
            total_objects: i32,
            detected_classes: Vec<String>,
            model: String,
            quality: String,
            frames_interval: i32,
            min_confidence: f32,
            processing_time: String,
            result_path: String,
        }

        if let Ok(legacy) = serde_json::from_str::<LegacyAnalysisResult>(&content) {
            return Ok(AnalysisResult {
                id: legacy.id,
                video_url: legacy.video_url,
                thumbnail: legacy.thumbnail,
                title: legacy.title,
                duration: legacy.duration,
                analyzed_date: legacy.analyzed_date,
                total_objects: legacy.total_objects,
                detected_classes: legacy.detected_classes,
                model: legacy.model,
                quality: legacy.quality,
                frames_interval: legacy.frames_interval,
                min_confidence: legacy.min_confidence,
                processing_time: legacy.processing_time,
                result_path: legacy.result_path,
                fps: 30,
                detections: Vec::new(),
                status: Some("complete".to_string()),
            });
        }
    }

    Err("Analysis not found".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            active_session: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_models_dir,
            get_current_models_dir,
            set_models_dir,
            list_downloaded_models,
            check_model_exists,
            download_model,
            delete_model,
            get_file_size,
            import_model,
            start_live_analysis,
            start_video_analysis,
            stop_video_analysis,
            get_active_analysis,
            get_all_analysis_results,
            get_analysis_result
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
