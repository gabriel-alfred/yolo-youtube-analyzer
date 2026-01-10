# YOLO YouTube Analyzer

**YOLO YouTube Analyzer** es una potente aplicación de escritorio construida con **Tauri, SvelteKit y Python** que permite realizar análisis de detección de objetos en videos de YouTube y transmisiones en tiempo real utilizando modelos **YOLOv11** y modelos que el usuario pueda importar.

## 🚀 Características Principales

*   **Dashboard Interactivo**: Visualización moderna con seguimiento ocular y acceso rápido a los análisis recientes.
*   **Análisis de Video**: Descarga y procesa videos de YouTube para detectar objetos.
    *   Soporte para múltiples clases (personas, vehículos, etc.).
    *   Configuración de confianza y dispositivo de procesamiento (CPU/GPU).
*   **Monitorización en Vivo**: Análisis en tiempo real de streams o webcams con superposición de detecciones.
*   **Gestión de Modelos**:
    *   Descarga automática de modelos YOLO (n, s, m, l, x).
    *   Importación de modelos personalizados (`.pt`).
*   **Historial de Resultados**: Galería visual de análisis anteriores con reproductor de video integrado y estadísticas detalladas.
*   **Internacionalización**: Interfaz completamente traducida al Español, Inglés y Euskera.
*   **Optimización**: Soporte para aceleración por GPU (CUDA/MPS) y gestión inteligente del almacenamiento.

## 🛠️ Requisitos del Sistema

Antes de ejecutar la aplicación, asegúrate de tener instalado:

1.  **Node.js** (v18 o superior) y **npm**.
2.  **Rust** (última versión estable) - Necesario para Tauri.
3.  **Python** (v3.10 o superior) accesible en el PATH.
    *   Dependencias de Python necesarias: `ultralytics`, `opencv-python`, `yt-dlp`, `torch`.
4.  **FFmpeg**: Esencial para la descarga y procesamiento de videos.
    *   Descarga desde [ffmpeg.org](https://ffmpeg.org/download.html).
    *   **Importante**: Asegúrate de añadir la carpeta `bin` de FFmpeg a las variables de entorno (PATH) de tu sistema.
5.  **Visual Studio C++ Build Tools** (en Windows) para compilar dependencias nativas.

## 📦 Instalación

1.  **Clonar el repositorio**:
    ```bash
    git clone https://github.com/tu-usuario/yolo-youtube-analyzer.git
    cd yolo-youtube-analyzer
    ```

2.  **Instalar dependencias de Frontend**:
    ```bash
    npm install
    ```

3.  **Configurar entorno Python**:
    La aplicación buscará `python` en el sistema. Asegúrate de instalar las librerías necesarias:
    ```bash
    pip install ultralytics opencv-python yt-dlp torch torchvision torchaudio
    ```
    *(Nota: Para soporte GPU, instala la versión de PyTorch adecuada para tu sistema).*

4.  **Ejecutar en modo Desarrollo**:
    ```bash
    npm run tauri dev
    ```

## 📖 Manual de Uso

### 1. Dashboard
La pantalla principal ofrece un resumen visual. Se muestra una cuadrícula con los **análisis más recientes**. Haz clic en cualquier tarjeta para ver los detalles.

### 2. Nuevo Análisis
Ve a la sección **"Nuevo Análisis"** en la barra lateral:
*   **URL de YouTube**: Pega un enlace para descargar y analizar.
*   **Archivo Local**: Selecciona un video de tu equipo.
*   **Configuración**: Elige el modelo YOLO, umbral de confianza y clases a detectar.

### 3. Monitorización (Live)
En la sección **"Monitorización"**:
*   Introduce una URL de stream o usa una cámara.
*   Selecciona el modelo y dispositivo.
*   Pulsa **"Iniciar Monitorización"** para ver detecciones en tiempo real.
*   *Nota: Si cierras la app mientras analizas, se te pedirá confirmación.*

### 4. Modelos
Gestiona tus modelos de IA:
*   Descarga variantes de YOLOv11 directamente desde la app.
*   Importa tus propios modelos `.pt` entrenados.
*   Visualiza el tamaño y estado de cada modelo.

### 5. Configuración
Ajusta preferencias globales:
*   **Idioma**: Español / Inglés / Euskera.
*   **Almacenamiento**: Gestiona el espacio usado por videos y modelos.
*   **Procesamiento**: Selecciona el dispositivo para el procesamiento (CPU/GPU).

## 🔧 Solución de Problemas

**Error: "Window close not allowed"**
Solucionado en la última versión. La aplicación ahora gestiona correctamente los permisos de cierre.

**Error: "Model not found" en Live Analysis**
Asegúrate de que el modelo seleccionado aparezca como "Descargado" en la sección de Modelos. El sistema ahora usa identificadores únicos para evitar confusiones.

**El análisis es lento**
Asegúrate de seleccionar **GPU (CUDA)** en la configuración de análisis si tienes una tarjeta gráfica NVIDIA compatible. El modo CPU es significativamente más lento.

## 🏗️ Construcción para Producción

Para generar el ejecutable instalable (`.exe` o `.msi`):

```bash
npm run tauri build
```
El instalador se generará en `src-tauri/target/release/bundle/nsis/`.

## 📊 Presentación del Proyecto

Puedes ver la presentación completa del proyecto con todos los detalles técnicos, arquitectura y casos de uso en el siguiente enlace:

[📥 Descargar/Ver Presentación PDF](./YOLO-YouTube-Analyzer.pdf)

## 📄 Licencia

Este proyecto está bajo la Licencia MIT.
