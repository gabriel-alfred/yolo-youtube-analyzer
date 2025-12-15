import argparse
import sys
import os
import json
import cv2
from ultralytics import YOLO
import yt_dlp
import base64
import time
import threading
import numpy as np

def parse_arguments():
    parser = argparse.ArgumentParser(description='YOLO Live Stream Analyzer')
    parser.add_argument('--url', type=str, required=True, help='YouTube Video URL')
    parser.add_argument('--model', type=str, required=True, help='Path to YOLO model')
    parser.add_argument('--conf', type=float, default=0.25, help='Confidence threshold')
    parser.add_argument('--classes', type=str, default=None, help='Comma separated class IDs')
    parser.add_argument('--device', type=str, default='cpu', help='Device (cpu, cuda, mps, etc)')
    parser.add_argument('--quality', type=str, default='medium', help='Stream quality (low, medium, high)')
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

def emit_progress(status, **kwargs):
    """Emite progreso al frontend de forma segura"""
    data = {"status": status}
    data.update(kwargs)
    
    print(json.dumps(data), flush=True)
    sys.stdout.flush()

def get_stream_url(url, quality='medium'):
    """Obtiene la URL del stream directo usando yt-dlp"""
    
    # Map quality to format selection
    format_selector = 'best'
    if quality == 'low':
        format_selector = 'worst[ext=mp4]/worst'
    elif quality == 'medium':
        format_selector = 'best[height<=720][ext=mp4]/best[height<=720]'
    elif quality == 'high':
        format_selector = 'best[ext=mp4]/best'

    ydl_opts = {
        'format': format_selector,
        'quiet': True,
        'no_warnings': True,
    }
    
    emit_progress("info", message="Obteniendo URL del stream...")
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            return info['url'], info.get('title', 'Unknown')
    except Exception as e:
        raise Exception(f"Error obteniendo stream: {str(e)}")

class VideoCaptureThread:
    def __init__(self, src):
        self.capture = cv2.VideoCapture(src)
        if not self.capture.isOpened():
            raise Exception("No se pudo abrir el stream de video")
        self.status = True
        self.frame = None
        self.frame_id = 0
        self.lock = threading.Lock()
        self.thread = threading.Thread(target=self.update, args=())
        self.thread.daemon = True
        self.thread.start()

    def update(self):
        while True:
            if self.capture.isOpened():
                (self.status, frame) = self.capture.read()
                if self.status:
                    with self.lock:
                        self.frame = frame
                        self.frame_id += 1
                else:
                    break
            else:
                break
            # Minimal sleep to yield CPU, but keep reading as fast as possible to drain buffer
            time.sleep(0.001)

    def read(self):
        with self.lock:
            return self.status, self.frame, self.frame_id

    def get_props(self):
        width = int(self.capture.get(cv2.CAP_PROP_FRAME_WIDTH))
        height = int(self.capture.get(cv2.CAP_PROP_FRAME_HEIGHT))
        fps = self.capture.get(cv2.CAP_PROP_FPS)
        return width, height, fps

    def release(self):
        self.status = False
        self.capture.release()

def analyze_stream(stream_url, model_path, conf, classes, device):
    """Analiza el stream en vivo"""
    cap_thread = None
    try:
        emit_progress("loading_model", message="Cargando modelo YOLO...")
        model = YOLO(model_path)
        
        emit_progress("connecting", message="Conectando al stream...")
        cap_thread = VideoCaptureThread(stream_url)
        
        # Wait for first frame
        retries = 0
        while cap_thread.frame is None and retries < 50:
            time.sleep(0.1)
            retries += 1
            
        if cap_thread.frame is None:
            raise Exception("No se pudo recibir video del stream")
            
        width, height, fps = cap_thread.get_props()
        if fps == 0 or np.isnan(fps): fps = 30.0
        
        emit_progress("started", message=f"Análisis iniciado: {width}x{height} @ {fps:.1f}fps")
        
        class_list = None
        if classes:
            try:
                class_list = [int(c) for c in classes.split(',')]
            except ValueError:
                class_list = None

        frame_count = 0
        
        # No artificial FPS limiting. We follow the stream's pace.
        # The capture thread drains the buffer. We just pick up the latest frame.
        
        last_processed_id = -1
        
        while cap_thread.status:
            # Get latest frame
            ret, frame, current_id = cap_thread.read()
            
            if not ret:
                break
                
            # If we already processed this frame ID, wait for a new one
            # This effectively syncs us to the capture thread's speed (which is the stream speed)
            if frame is None or current_id == last_processed_id:
                time.sleep(0.001) # Minimal sleep to yield CPU
                continue
            
            last_processed_id = current_id
            frame_count += 1
            
            # Run inference
            results = model(
                frame, 
                conf=conf,
                classes=class_list,
                device=device,
                verbose=False
            )
            
            # Draw results
            result_frame = results[0].plot()
            
            # Resize for transmission
            preview_height = 480
            scale = preview_height / result_frame.shape[0]
            preview_width = int(result_frame.shape[1] * scale)
            preview_frame = cv2.resize(result_frame, (preview_width, preview_height))
            
            # Encode to base64
            _, buffer = cv2.imencode('.jpg', preview_frame, [cv2.IMWRITE_JPEG_QUALITY, 70])
            frame_base64 = base64.b64encode(buffer).decode('utf-8')
            
            # Collect stats
            detections = []
            for box in results[0].boxes:
                cls_id = int(box.cls[0])
                class_name = results[0].names[cls_id]
                confidence = float(box.conf[0])
                detections.append(f"{class_name} ({confidence:.2f})")
            
            # Emit frame
            emit_progress(
                "frame",
                frame_data=frame_base64,
                frame_number=current_id,
                detections=detections,
                fps=fps
            )
            
            # No sleep here. We are ready for the next frame immediately.
            # If the next frame hasn't arrived yet, the loop top will wait.
            
    except Exception as e:
        emit_progress("error", message=f"Error en análisis: {str(e)}")
    finally:
        if cap_thread:
            cap_thread.release()

def main():
    try:
        args = parse_arguments()
        
        emit_progress("starting", message="Iniciando componente de análisis en vivo...")
        
        validated_device = validate_device(args.device)
        
        stream_url, title = get_stream_url(args.url, args.quality)
        emit_progress("info", message=f"Stream encontrado: {title}")
        
        analyze_stream(
            stream_url,
            args.model,
            args.conf,
            args.classes,
            validated_device
        )
        
    except Exception as e:
        emit_progress("error", message=str(e))
        sys.exit(1)

if __name__ == "__main__":
    main()
