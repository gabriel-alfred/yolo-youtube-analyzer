<script>
  import { onMount, onDestroy } from 'svelte';
  import Card from '$lib/components/ui/Card.svelte';

  let pupilX = 30;
  let pupilY = 0;
  let targetX = 30;
  let targetY = 0;
  let scanInterval;
  let targetInterval;
  let scanPhase = 0;
  let isVerticalLayout = false;
  
  function checkLayout() {
    isVerticalLayout = window.innerWidth < 1024;
  }
  
  function setNewTarget() {
    if (isVerticalLayout) {
      // Movimientos principalmente hacia abajo cuando está en layout vertical
      // Con la rotación de 90°, Y se convierte en el movimiento hacia abajo
      targetX = 25 + Math.random() * 15; // Más a la derecha (tras rotación es centrado verticalmente)
      targetY = -5 + Math.random() * 25; // Un poco más arriba y principalmente hacia abajo
    } else {
      // Movimientos principalmente hacia la derecha en layout horizontal
      targetX = 15 + Math.random() * 35;
      targetY = -20 + Math.random() * 40;
    }
    
    // Limitar el objetivo dentro del ojo
    const maxDistance = 32;
    const distance = Math.sqrt((targetX - 30) * (targetX - 30) + targetY * targetY);
    if (distance > maxDistance) {
      targetX = 30 + ((targetX - 30) / distance) * maxDistance;
      targetY = (targetY / distance) * maxDistance;
    }
  }
  
  function scanMovement() {
    scanPhase += 0.055;
    
    // Movimiento suave hacia el objetivo (sacádico)
    const dx = targetX - pupilX;
    const dy = targetY - pupilY;
    
    // Interpolación suave pero rápida (más humano)
    pupilX += dx * 0.15;
    pupilY += dy * 0.15;
    
    // Micro-movimientos (tremor ocular)
    pupilX += (Math.random() - 0.5) * 0.8;
    pupilY += (Math.random() - 0.5) * 0.8;
  }
  
  onMount(() => {
    // Verificar el layout inicial
    checkLayout();
    
    // Escuchar cambios de tamaño de ventana
    window.addEventListener('resize', () => {
      checkLayout();
      setNewTarget();
    });
    
    // Establecer primer objetivo
    setNewTarget();
    
    // Movimiento continuo
    scanInterval = setInterval(scanMovement, 50);
    
    // Cambiar de objetivo cada 1.5-4 segundos (movimientos sacádicos)
    targetInterval = setInterval(() => {
      setNewTarget();
    }, 1500 + Math.random() * 2500);
  });
  
  onDestroy(() => {
    if (scanInterval) clearInterval(scanInterval);
    if (targetInterval) clearInterval(targetInterval);
  });
</script>

<div class="w-full min-h-screen bg-transparent flex flex-col items-center pt-12 px-16 gap-8 relative overflow-hidden">
  <!-- Título y descripción -->
  <div class="text-center mb-8">
    <h1 class="text-4xl font-bold text-orange-500 mb-3">YOLO Analizador de Video de YouTube</h1>
    <p class="text-slate-300 text-lg max-w-2xl">
      Sistema de análisis visual en tiempo real con tecnología de seguimiento ocular avanzado para procesamiento inteligente de contenido multimedia
    </p>
  </div>

  <!-- Contenedor principal centrado con diseño responsivo -->
  <div class="flex flex-col lg:flex-row items-center justify-center gap-16 w-full max-w-6xl">
    <!-- Lado izquierdo: Eye -->
    <div class="flex items-center justify-center w-full lg:w-auto">
      <svg 
        viewBox="0 0 220 200" 
        width="280" 
        height="280" 
        class="mx-auto transition-transform duration-500"
        style="transform: rotate({isVerticalLayout ? 90 : 0}deg);"
      >
      <defs>
        <!-- Gradientes para 3D -->
        <radialGradient id="glow">
          <stop offset="0%" stop-color="rgba(249, 115, 22, 0.8)"/>
          <stop offset="50%" stop-color="rgba(249, 115, 22, 0.3)"/>
          <stop offset="100%" stop-color="rgba(249, 115, 22, 0)"/>
        </radialGradient>
        
        <radialGradient id="iris">
          <stop offset="0%" stop-color="#fbbf24"/>
          <stop offset="30%" stop-color="#f97316"/>
          <stop offset="70%" stop-color="#ea580c"/>
          <stop offset="100%" stop-color="#c2410c"/>
        </radialGradient>
        
        <linearGradient id="techLine" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stop-color="rgba(249, 115, 22, 0)"/>
          <stop offset="50%" stop-color="rgba(249, 115, 22, 1)"/>
          <stop offset="100%" stop-color="rgba(249, 115, 22, 0)"/>
        </linearGradient>
        
        <radialGradient id="pupil">
          <stop offset="0%" stop-color="#431407"/>
          <stop offset="100%" stop-color="#000"/>
        </radialGradient>

        <!-- Gradiente para esclerótica con aspecto tecnológico oscuro -->
        <radialGradient id="eyeWhite" cx="25%" cy="50%">
          <stop offset="0%" stop-color="#1e293b"/>
          <stop offset="70%" stop-color="#0f172a"/>
          <stop offset="100%" stop-color="#020617"/>
        </radialGradient>

        <filter id="shadow">
          <feDropShadow dx="3" dy="2" stdDeviation="5" flood-opacity="0.5"/>
        </filter>

        <!-- Filtro adicional para sombra lateral 3D (luz desde izquierda) -->
        <filter id="sideShadow">
          <feDropShadow dx="5" dy="0" stdDeviation="3" flood-color="rgba(0,0,0,0.3)"/>
        </filter>

        <filter id="neon">
          <feGaussianBlur stdDeviation="4" result="coloredBlur"/>
          <feMerge>
            <feMergeNode in="coloredBlur"/>
            <feMergeNode in="SourceGraphic"/>
          </feMerge>
        </filter>

        <filter id="strongGlow">
          <feGaussianBlur stdDeviation="6" result="coloredBlur"/>
          <feMerge>
            <feMergeNode in="coloredBlur"/>
            <feMergeNode in="coloredBlur"/>
            <feMergeNode in="SourceGraphic"/>
          </feMerge>
        </filter>

        <!-- Máscara para forma de ojo en perspectiva 3D suave (izquierda estrecha, derecha ancha, curvas naturales) -->
        <clipPath id="eyeShape">
          <path d="M 10,100 Q 35,88 55,82 Q 75,78 105,78 Q 135,78 165,84 Q 185,90 200,100 Q 185,110 165,116 Q 135,122 105,122 Q 75,122 55,118 Q 35,112 10,100 Z" />
        </clipPath>
      </defs>

      <!-- Resplandor exterior más intenso -->
      <circle cx="110" cy="100" r="95" fill="url(#glow)"/>
      <circle cx="110" cy="100" r="70" fill="url(#glow)" opacity="0.4"/>

      <!-- Borde tecnológico exterior -->
      <circle 
        cx="110" 
        cy="100" 
        r="95"
        fill="none"
        stroke="url(#techLine)"
        stroke-width="2"
        filter="url(#neon)"
        opacity="0.8"
      />
      
      <!-- Borde tecnológico con segmentos -->
      <circle 
        cx="110" 
        cy="100" 
        r="98"
        fill="none"
        stroke="#f97316"
        stroke-width="1.5"
        stroke-dasharray="8 4"
        filter="url(#neon)"
        opacity="0.6"
        transform="rotate({scanPhase * 20} 110 100)"
      />
      
      <!-- Anillo exterior con segmentos HUD -->
      <circle 
        cx="110" 
        cy="100" 
        r="102"
        fill="none"
        stroke="rgba(249, 115, 22, 0.5)"
        stroke-width="0.8"
        stroke-dasharray="3 2"
        filter="url(#neon)"
      />
      
      <!-- Marcadores de esquina tecnológicos -->
      {#each [0, 90, 180, 270] as angle}
        {@const rad = (angle * Math.PI) / 180}
        {@const x = 110 + Math.cos(rad) * 105}
        {@const y = 100 + Math.sin(rad) * 105}
        <g transform="rotate({angle} {x} {y})">
          <line 
            x1={x - 8} y1={y} x2={x - 3} y2={y}
            stroke="#f97316"
            stroke-width="2"
            filter="url(#neon)"
          />
          <line 
            x1={x} y1={y - 8} x2={x} y2={y - 3}
            stroke="#f97316"
            stroke-width="2"
            filter="url(#neon)"
          />
        </g>
      {/each}

      <!-- Grupo sin clip path para que se vea todo el ojo -->
      <g>
        <!-- Parte blanca del ojo (esclerótica) oscura tecnológica -->
        <circle 
          cx="110" 
          cy="100" 
          r="95"
          fill="url(#eyeWhite)"
          filter="url(#shadow)"
          class="eye-part"
        />

        <!-- Circuitos tecnológicos en la esclerótica -->
        {#each Array(16) as _, i}
          {@const angle = (i / 16) * Math.PI * 2}
          {@const startRadius = 45}
          {@const endRadius = 90}
          {@const x1 = 110 + Math.cos(angle) * startRadius}
          {@const y1 = 100 + Math.sin(angle) * startRadius}
          {@const x2 = 110 + Math.cos(angle) * endRadius}
          {@const y2 = 100 + Math.sin(angle) * endRadius}
          <line 
            {x1} {y1} {x2} {y2} 
            stroke="rgba(249, 115, 22, 0.15)" 
            stroke-width="0.5"
          />
        {/each}

        <!-- Anillos de circuito concéntricos -->
        {#each [50, 65, 80] as radius}
          <circle 
            cx="110" 
            cy="100" 
            r={radius}
            fill="none"
            stroke="rgba(249, 115, 22, 0.2)"
            stroke-width="0.5"
            stroke-dasharray="4 4"
          />
        {/each}

        <!-- Iris -->
        <g class="eye-part">
          <!-- Iris exterior con efecto neón intenso y perspectiva -->
          <ellipse 
            cx={150 + pupilX * 0.4} 
            cy={100 + pupilY}
            rx="28" 
            ry="35"
            fill="url(#iris)"
            filter="url(#strongGlow)"
          />

          <!-- Círculos concéntricos del iris con perspectiva y brillo -->
          {#each [24, 18, 12] as radius, i}
            <ellipse 
              cx={150 + pupilX * 0.4} 
              cy={100 + pupilY}
              rx={radius * 0.8}
              ry={radius * 1.2}
              fill="none"
              stroke="rgba(251, 191, 36, {0.6 - i * 0.15})"
              stroke-width="2"
              filter="url(#neon)"
            />
          {/each}

          <!-- Detalles radiales del iris que rotan con el movimiento - más brillantes -->
          {#each Array(28) as _, i}
            {@const angle = (i / 28) * Math.PI * 2 + scanPhase * 0.3}
            {@const x1 = 150 + pupilX * 0.4 + Math.cos(angle) * 8 * 0.8}
            {@const y1 = 100 + pupilY + Math.sin(angle) * 8 * 1.2}
            {@const x2 = 150 + pupilX * 0.4 + Math.cos(angle) * 25 * 0.8}
            {@const y2 = 100 + pupilY + Math.sin(angle) * 35 * 1.2}
            <line {x1} {y1} {x2} {y2} stroke="rgba(251, 191, 36, 0.5)" stroke-width="1"/>
          {/each}

          <!-- Patrón de datos binarios rotando -->
          {#each Array(6) as _, ring}
            {@const ringRadius = 10 + ring * 2.5}
            {#each Array(20) as _, i}
              {@const angle = (i / 20) * Math.PI * 2 + scanPhase * 0.2 + ring * 0.3}
              {@const x = 150 + pupilX * 0.4 + Math.cos(angle) * ringRadius * 0.8}
              {@const y = 100 + pupilY + Math.sin(angle) * ringRadius * 1.2}
              <circle 
                cx={x}
                cy={y}
                r="0.8"
                fill="rgba(251, 191, 36, {0.7 - ring * 0.08})"
              />
            {/each}
          {/each}

          <!-- Pupila con rotación y perspectiva -->
          <ellipse 
            cx={150 + pupilX * 0.4} 
            cy={100 + pupilY}
            rx="10" 
            ry="14"
            fill="url(#pupil)"
          />

          <!-- HUD - Líneas de escaneo cruzadas -->
          <line
            x1={150 + pupilX * 0.4 - 15}
            y1={100 + pupilY}
            x2={150 + pupilX * 0.4 + 15}
            y2={100 + pupilY}
            stroke="#f97316"
            stroke-width="0.5"
            opacity="0.6"
          />
          <line
            x1={150 + pupilX * 0.4}
            y1={100 + pupilY - 20}
            x2={150 + pupilX * 0.4}
            y2={100 + pupilY + 20}
            stroke="#f97316"
            stroke-width="0.5"
            opacity="0.6"
          />

          <!-- Círculo de targeting -->
          <circle 
            cx={150 + pupilX * 0.4} 
            cy={100 + pupilY}
            r="18"
            fill="none"
            stroke="#f97316"
            stroke-width="0.8"
            stroke-dasharray="2 3"
            opacity="0.5"
            transform="rotate({scanPhase * 30} {150 + pupilX * 0.4} {100 + pupilY})"
          />

          <!-- Anillo interior de la pupila con brillo tecnológico -->
          <ellipse 
            cx={150 + pupilX * 0.4} 
            cy={100 + pupilY}
            rx="11"
            ry="15"
            fill="none"
            stroke="rgba(249, 115, 22, 0.8)"
            stroke-width="1.5"
            filter="url(#neon)"
          />

          <!-- Brillo en la pupila para profundidad -->
          <circle 
            cx={150 + pupilX * 0.4 - 3} 
            cy={100 + pupilY - 3.5}
            r="2.5"
            fill="rgba(254, 215, 170, 0.9)"
          />
          <circle 
            cx={150 + pupilX * 0.4 - 3.2} 
            cy={100 + pupilY - 3.8}
            r="1.2"
            fill="rgba(255, 255, 255, 1)"
          />
        </g>
      </g>
    </svg>
    </div>

    <!-- Lado derecho: Video Card -->
    <div class="w-full lg:w-auto">
      <Card variant="glass" padding="md" hoverable={false}>
        <div class="space-y-4">
          
          <div class="relative rounded-lg overflow-hidden border border-orange-500/30 shadow-lg shadow-orange-500/20">
            <video 
              class="w-full h-auto"
              src="/video.mp4"
              autoplay
              loop
              muted
              playsinline
            >
              Tu navegador no soporta el elemento de video.
            </video>
          </div>
        </div>
      </Card>
    </div>

  </div>

  <!-- Dashboard de Análisis Fijados -->
  <div class="w-full max-w-7xl mt-16 mb-12">
    <div class="text-center mb-8">
      <h2 class="text-3xl font-bold text-orange-500 mb-2">Dashboard de Análisis</h2>
      <p class="text-slate-400">Selecciona y fija tus análisis favoritos para acceso rápido</p>
    </div>

    <!-- Grid de 2 filas x 3 columnas -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each Array(6) as _, i}
        <Card variant="glass" padding="md" hoverable={true}>
          <div class="space-y-3">

            <!-- Área de video/placeholder -->
            <div class="relative aspect-video rounded-lg overflow-hidden border border-orange-500/20 bg-slate-900/50 flex items-center justify-center group cursor-pointer hover:border-orange-500/50 transition-colors">
              <div class="text-center">
                <svg class="w-12 h-12 text-orange-500/30 mx-auto mb-2 group-hover:text-orange-500/50 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                </svg>
                <p class="text-xs text-slate-500 group-hover:text-slate-400 transition-colors">Añadir análisis</p>
              </div>
            </div>

          </div>
        </Card>
      {/each}
    </div>
  </div>
</div>

<style>
  .eye-part {
    transition: all 0.15s ease-in-out;
  }
</style>