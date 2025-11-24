import argparse
import sys
import os
import json
import cv2
from ultralytics import YOLO
import yt_dlp
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import threading
import base64
import shutil
import time
import subprocess
import tempfile
from datetime import datetime

def parse_arguments():
    parser = argparse.ArgumentParser(description='YOLO Video Analyzer')
    parser.add_argument('--url', type=str, required=True, help='YouTube Video URL')
    parser.add_argument('--model', type=str, required=True, help='Path to YOLO model')
    parser.add_argument('--output_dir', type=str, required=True, help='Directory to save results')
    parser.add_argument('--conf', type=float, default=0.25, help='Confidence threshold')
    parser.add_argument('--classes', type=str, default=None, help='Comma separated class IDs')
    parser.add_argument('--device', type=str, default='cpu', help='Device (cpu, cuda, mps, etc)')
    parser.add_argument('--frames', type=int, default=1, help='Process every N frames')
    parser.add_argument('--quality', type=int, default=80, help='Output quality (50-100)')
    return parser.parse_args()

def validate_device(device_str):
    """Valida y normaliza el device string para YOLO"""
    device_str = device_str.lower().strip()
    
    device_map = {
        'cpu': 'cpu',
        'cuda': '0',
        'gpu cuda': '0',
        'gpu': '0',
        'mps': 'mps',
        'gpu apple silicon': 'mps',
        '0': '0',
    }
    
    if device_str in device_map:
        return device_map[device_str]
    
    if device_str.isdigit():
        return device_str
    
    return 'cpu'

def emit_progress(status, progress=None, **kwargs):
    """Emite progreso al frontend de forma segura"""
    data = {"status": status}
    if progress is not None:
        data["progress"] = progress
    data.update(kwargs)
    
    print(json.dumps(data), flush=True)
    sys.stdout.flush()
    
    if status == "frame":
        time.sleep(0.01)

def download_video(url, output_dir):
    """Descarga un video de YouTube y retorna path y metadata"""
    ydl_opts = {
        'format': 'best[ext=mp4]',
        'outtmpl': os.path.join(output_dir, '%(id)s.%(ext)s'),
        'quiet': True,
        'no_warnings': True,
    }
    
    emit_progress("downloading", progress=0, message="Descargando video...")
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=True)
            filename = ydl.prepare_filename(info)
            
            # Extraer metadata relevante
            metadata = {
                'title': info.get('title', 'Unknown Title'),
                'thumbnail': info.get('thumbnail', ''),
                'duration': info.get('duration_string', '0:00'),
                'upload_date': info.get('upload_date', ''),
                'uploader': info.get('uploader', ''),
                'video_id': info.get('id', ''),
                'original_url': url
            }
            
    except Exception as e:
        raise Exception(f"Error descargando video: {str(e)}")
    
    emit_progress("download_complete", progress=10, message="Video descargado")
    return filename, metadata

def check_ffmpeg():
    """Verifica si FFmpeg está disponible"""
    try:
        subprocess.run(['ffmpeg', '-version'], 
                      stdout=subprocess.PIPE, 
                      stderr=subprocess.PIPE,
                      check=True)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False

def save_analysis_metadata(output_path, metadata, stats, config):
    """Guarda los metadatos del análisis en un archivo JSON"""
    json_path = os.path.splitext(output_path)[0] + '.json'
    
    analysis_data = {
        'id': metadata['video_id'],
        'videoUrl': metadata['original_url'],
        'thumbnail': metadata['thumbnail'],
        'title': metadata['title'],
        'duration': metadata['duration'],
        'analyzedDate': datetime.now().isoformat(),
        'totalObjects': stats['total_objects'],
        'detectedClasses': list(stats['detected_classes']),
        'model': os.path.basename(config['model']),
        'quality': f"{config['quality']}p" if config['quality'] > 0 else "Auto",
        'framesInterval': config['frames'],
        'minConfidence': config['conf'],
        'processingTime': stats['processing_time'],
        'resultPath': output_path,
        'fps': config['fps'],
        'detections': stats['detections']
    }
    
    with open(json_path, 'w', encoding='utf-8') as f:
        json.dump(analysis_data, f, indent=2, ensure_ascii=False)
        
    return json_path

def analyze_video(video_path, model_path, output_dir, conf, classes, device, frames_skip, quality, metadata):
    """Analiza un video con YOLO con streaming de frames"""
    start_time = time.time()
    try:
        emit_progress("loading_model", progress=10, message="Cargando modelo...")
        model = YOLO(model_path)
        
        emit_progress("opening_video", progress=15, message="Abriendo video...")
        cap = cv2.VideoCapture(video_path)
        if not cap.isOpened():
            raise Exception("No se pudo abrir el archivo de video")
        
        width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
        height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))
        fps = int(cap.get(cv2.CAP_PROP_FPS))
        total_frames = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
        
        if fps == 0:
            fps = 30
        if total_frames == 0:
            total_frames = 1
        
        output_filename = f"analyzed_{os.path.splitext(os.path.basename(video_path))[0]}.mp4"
        output_path = os.path.join(output_dir, output_filename)
        last_emit_time = 0
        temp_dir = None
        
        # Stats tracking
        stats = {
            'total_objects': 0,
            'detected_classes': set(),
            'processing_time': '',
            'detections': []
        }
        
        # Estrategia: Usar FFmpeg si está disponible, sino intentar OpenCV con mp4v
        use_ffmpeg = check_ffmpeg()
        
        if use_ffmpeg:
            emit_progress("info", message="Usando FFmpeg para codificación (mejor calidad)")
            # Crear directorio temporal para frames
            temp_dir = tempfile.mkdtemp()
            temp_pattern = os.path.join(temp_dir, "frame_%06d.jpg")
            out = None
            used_codec = "FFmpeg H.264"
        else:
            emit_progress("warning", message="FFmpeg no disponible. Instálalo para mejor compatibilidad: https://ffmpeg.org/download.html")
            temp_dir = None
            # Intentar codecs compatibles con navegadores
            fourcc_codes = [
                ('avc1', 'H.264 (AVC1)'),
                ('H264', 'H.264'),
                ('mp4v', 'MPEG-4'),
                ('MJPG', 'Motion JPEG'),
            ]
            
            out = None
            used_codec = None
            
            # Suppress OpenCV stderr output during initialization to avoid frontend errors
            with open(os.devnull, 'w') as devnull:
                old_stderr = sys.stderr
                sys.stderr = devnull
                try:
                    for fourcc_str, codec_name in fourcc_codes:
                        try:
                            fourcc = cv2.VideoWriter_fourcc(*fourcc_str)
                            test_out = cv2.VideoWriter(output_path, fourcc, fps, (width, height))
                            
                            if test_out.isOpened():
                                out = test_out
                                used_codec = codec_name
                                break
                            else:
                                test_out.release()
                        except Exception:
                            continue
                finally:
                    sys.stderr = old_stderr
            
            if used_codec:
                emit_progress("info", message=f"Usando codec: {used_codec}")

            if not out or not out.isOpened():
                raise Exception("No se pudo crear el archivo de video. Instala FFmpeg para mejor compatibilidad: https://ffmpeg.org/download.html")

        class_list = None
        if classes:
            try:
                class_list = [int(c) for c in classes.split(',')]
                emit_progress("info", message=f"Detectando clases: {class_list}")
            except ValueError:
                class_list = None
        
        emit_progress("analyzing", progress=20, message="Iniciando análisis...")
        
        frames_lock = threading.Lock()
        stats_lock = threading.Lock()
        
        def process_frame(frame_data):
            """Procesa un frame con YOLO"""
            frame, frame_num = frame_data
            try:
                results = model(
                    frame, 
                    conf=conf,
                    classes=class_list,
                    device=device,
                    verbose=False
                )
                
                # Update stats
                result = results[0]
                
                # Calculate timestamp
                timestamp_sec = frame_num / fps
                minutes = int(timestamp_sec // 60)
                seconds = int(timestamp_sec % 60)
                timestamp_str = f"{minutes:02d}:{seconds:02d}"
                
                frame_detections = []
                for box in result.boxes:
                    cls_id = int(box.cls[0])
                    class_name = result.names[cls_id]
                    confidence = float(box.conf[0])
                    xywh = box.xywh[0].tolist()
                    
                    frame_detections.append({
                        'frameNumber': frame_num,
                        'timestamp': timestamp_str,
                        'class': class_name,
                        'confidence': confidence,
                        'bbox': {
                            'x': int(xywh[0]),
                            'y': int(xywh[1]),
                            'width': int(xywh[2]),
                            'height': int(xywh[3])
                        }
                    })

                with stats_lock:
                    stats['total_objects'] += len(result.boxes)
                    for cls_id in result.boxes.cls:
                        class_name = result.names[int(cls_id)]
                        stats['detected_classes'].add(class_name)
                    stats['detections'].extend(frame_detections)
                
                return result.plot()
            except Exception as e:
                emit_progress("error", message=f"Error en frame: {str(e)}")
                return frame
        
        def save_and_emit_frame(result_frame, frame_num):
            """Guarda y emite un frame para preview"""
            nonlocal saved_frame_count, last_emit_time
            
            with frames_lock:
                saved_frame_count += 1
                current_time = time.time()
                
                # Guardar frame
                if use_ffmpeg:
                    # Guardar como imagen temporal
                    frame_path = os.path.join(temp_dir, f"frame_{saved_frame_count:06d}.jpg")
                    cv2.imwrite(frame_path, result_frame, [cv2.IMWRITE_JPEG_QUALITY, 95])
                else:
                    # Escribir directamente con OpenCV
                    out.write(result_frame)
                
                # Emitir preview
                should_emit = (saved_frame_count % 3 == 0) or (current_time - last_emit_time > 0.2)
                
                if should_emit:
                    try:
                        preview_height = 480
                        preview_width = int(result_frame.shape[1] * (preview_height / result_frame.shape[0]))
                        preview_frame = cv2.resize(result_frame, (preview_width, preview_height))
                        
                        encode_param = [int(cv2.IMWRITE_JPEG_QUALITY), 70]
                        success, buffer = cv2.imencode('.jpg', preview_frame, encode_param)
                        
                        if success:
                            frame_base64 = base64.b64encode(buffer).decode('utf-8')
                            current_progress = int(20 + (frame_num / max(total_frames, 1)) * 75)
                            
                            emit_progress("frame", 
                                        progress=min(current_progress, 95),
                                        frame_data=frame_base64, 
                                        frame_number=saved_frame_count,
                                        total_frames=total_frames)
                            
                            last_emit_time = current_time
                    except Exception as e:
                        pass
        
        # Procesar video
        max_workers = 4
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            frame_batch = []
            frame_indices = []
            
            saved_frame_count = 0
            frame_count = 0
            processed_count = 0
            while cap.isOpened():
                ret, frame = cap.read()
                if not ret:
                    if frame_batch:
                        # Pass tuple of (frame, frame_num) to process_frame
                        batch_data = list(zip(frame_batch, frame_indices))
                        results = list(executor.map(process_frame, batch_data))
                        for idx, result_frame in enumerate(results):
                            save_and_emit_frame(result_frame, frame_indices[idx])
                            progress = int(20 + (frame_indices[idx] / max(total_frames, 1)) * 75)
                            emit_progress("analyzing", progress=min(progress, 95), 
                                         message=f"Procesando frame {frame_indices[idx]}/{total_frames}")
                    break
                
                frame_count += 1
                
                if frame_count % frames_skip != 0:
                    if use_ffmpeg:
                        frame_path = os.path.join(temp_dir, f"frame_{saved_frame_count + 1:06d}.jpg")
                        cv2.imwrite(frame_path, frame, [cv2.IMWRITE_JPEG_QUALITY, 95])
                    else:
                        out.write(frame)
                    saved_frame_count += 1
                    continue
                
                processed_count += 1
                frame_batch.append(frame)
                frame_indices.append(frame_count)
                
                if len(frame_batch) >= max_workers:
                    # Pass tuple of (frame, frame_num) to process_frame
                    batch_data = list(zip(frame_batch, frame_indices))
                    results = list(executor.map(process_frame, batch_data))
                    for idx, result_frame in enumerate(results):
                        save_and_emit_frame(result_frame, frame_indices[idx])
                        progress = int(20 + (frame_indices[idx] / max(total_frames, 1)) * 75)
                        if saved_frame_count % 30 == 0:
                            emit_progress("analyzing", progress=min(progress, 95), 
                                         message=f"Procesando frame {frame_indices[idx]}/{total_frames}")
                    frame_batch = []
                    frame_indices = []
        
        cap.release()
        if not use_ffmpeg:
            out.release()
        
        emit_progress("saving", progress=95, message="Finalizando video...")
        
        # Si usamos FFmpeg, codificar ahora
        if use_ffmpeg:
            try:
                emit_progress("info", message="Codificando video con FFmpeg...")
                cmd = [
                    'ffmpeg',
                    '-y',  # Sobrescribir
                    '-framerate', str(fps),
                    '-i', temp_pattern,
                    '-c:v', 'libx264',
                    '-preset', 'medium',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    '-movflags', '+faststart',  # Importante para reproducción en navegador
                    output_path
                ]
                
                result = subprocess.run(cmd, 
                                       stdout=subprocess.PIPE, 
                                       stderr=subprocess.PIPE,
                                       check=True)
                
                # Limpiar archivos temporales
                shutil.rmtree(temp_dir)
                
            except subprocess.CalledProcessError as e:
                raise Exception(f"Error codificando con FFmpeg: {e.stderr.decode()}")
        
        time.sleep(0.5)
        
        if not os.path.exists(output_path):
            raise Exception("El archivo de salida no se creó correctamente")
        
        file_size = os.path.getsize(output_path)
        if file_size == 0:
            raise Exception("El archivo de salida está vacío")
        
        # Si no se usó FFmpeg pero está disponible, re-encodear para compatibilidad
        if not use_ffmpeg and check_ffmpeg() and used_codec not in ["FFmpeg H.264", "H.264 (AVC1)", "H.264"]:
            emit_progress("info", message="Re-codificando video para compatibilidad con navegadores...")
            temp_output = output_path + ".temp.mp4"
            
            try:
                cmd = [
                    'ffmpeg',
                    '-y',
                    '-i', output_path,
                    '-c:v', 'libx264',
                    '-preset', 'medium',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    '-movflags', '+faststart',  # Optimizar para streaming
                    temp_output
                ]
                
                subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
                
                # Reemplazar archivo original
                os.remove(output_path)
                os.rename(temp_output, output_path)
                used_codec = "H.264 (re-encoded)"
                file_size = os.path.getsize(output_path)
                
                emit_progress("info", message="Video re-codificado con éxito")
            except Exception as e:
                emit_progress("warning", message=f"No se pudo re-codificar: {str(e)}")
                if os.path.exists(temp_output):
                    os.remove(temp_output)
        
        # Calculate processing time
        end_time = time.time()
        duration_sec = int(end_time - start_time)
        minutes = duration_sec // 60
        seconds = duration_sec % 60
        stats['processing_time'] = f"{minutes}m {seconds}s"
        
        # Save metadata
        config = {
            'model': model_path,
            'quality': quality,
            'frames': frames_skip,
            'conf': conf,
            'fps': fps
        }
        json_path = save_analysis_metadata(output_path, metadata, stats, config)
        
        emit_progress("info", message=f"Video guardado: {output_path} ({file_size} bytes, codec: {used_codec})")
        emit_progress("info", message=f"Metadatos guardados: {json_path}")

        return output_path
        
    except Exception as e:
        if temp_dir and os.path.exists(temp_dir):
            shutil.rmtree(temp_dir)
        raise Exception(f"Error analizando video: {str(e)}")

def main():
    try:
        args = parse_arguments()
        
        if not os.path.exists(args.output_dir):
            os.makedirs(args.output_dir)
        
        emit_progress("starting", progress=0, message="Iniciando proceso...")
        
        validated_device = validate_device(args.device)
        
        emit_progress("config_received", progress=5, 
                     message=f"Configuración: conf={args.conf}, device={validated_device}, frames={args.frames}")
        
        video_path, metadata = download_video(args.url, args.output_dir)
        
        output_path = analyze_video(
            video_path, 
            args.model, 
            args.output_dir, 
            args.conf,
            args.classes,
            validated_device,
            args.frames,
            args.quality,
            metadata
        )
        
        emit_progress("complete", progress=100, result_path=output_path, message="¡Análisis completado!")
        time.sleep(0.5)
        
    except Exception as e:
        emit_progress("error", message=str(e))
        sys.exit(1)

if __name__ == "__main__":
    main()