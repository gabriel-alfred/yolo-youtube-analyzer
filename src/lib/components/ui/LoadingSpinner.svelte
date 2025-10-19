<script lang="ts">
  type Size = 'sm' | 'md' | 'lg' | 'xl';
  type Variant = 'default' | 'primary' | 'glass';
  
  interface Props {
    size?: Size;
    variant?: Variant;
    message?: string;
    fullScreen?: boolean;
    class?: string;
  }
  
  let {
    size = 'md',
    variant = 'default',
    message,
    fullScreen = false,
    class: className = ''
  }: Props = $props();
  
  const sizeClasses: Record<Size, { spinner: string; text: string }> = {
    sm: { spinner: 'w-6 h-6', text: 'text-xs' },
    md: { spinner: 'w-10 h-10', text: 'text-sm' },
    lg: { spinner: 'w-16 h-16', text: 'text-base' },
    xl: { spinner: 'w-24 h-24', text: 'text-lg' }
  };
  
  const variantClasses: Record<Variant, string> = {
    default: 'text-red-500',
    primary: 'text-orange-500',
    glass: 'text-red-400'
  };
  
  const containerClass = fullScreen 
    ? 'fixed inset-0 z-50 flex items-center justify-center bg-slate-950/80 backdrop-blur-sm' 
    : 'flex items-center justify-center';
</script>

<div class="{containerClass} {className}">
  <div class="flex flex-col items-center gap-4">
    <!-- Spinner with glow effect -->
    <div class="relative">
      <!-- Outer glow -->
      <div class="absolute inset-0 {sizeClasses[size].spinner}">
        <div class="absolute inset-0 bg-gradient-to-r from-red-500 to-orange-500 rounded-full blur-xl opacity-50 animate-pulse"></div>
      </div>
      
      <!-- Spinner -->
      <svg 
        class="{sizeClasses[size].spinner} {variantClasses[variant]} animate-spin relative z-10" 
        xmlns="http://www.w3.org/2000/svg" 
        fill="none" 
        viewBox="0 0 24 24"
      >
        <circle 
          class="opacity-25" 
          cx="12" 
          cy="12" 
          r="10" 
          stroke="currentColor" 
          stroke-width="4"
        ></circle>
        <path 
          class="opacity-75" 
          fill="currentColor" 
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        ></path>
      </svg>
    </div>
    
    <!-- Message -->
    {#if message}
      <p class="{sizeClasses[size].text} font-medium text-red-100/80 animate-pulse">
        {message}
      </p>
    {/if}
  </div>
</div>