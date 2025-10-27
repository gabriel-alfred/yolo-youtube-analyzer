<script lang="ts">
  type Variant = 'error' | 'warning' | 'info' | 'success';
  
  interface Props {
    variant?: Variant;
    title?: string;
    message: string;
    dismissible?: boolean;
    onDismiss?: () => void;
    icon?: import('svelte').Snippet;
    actions?: import('svelte').Snippet;
    class?: string;
  }
  
  let {
    variant = 'error',
    title,
    message,
    dismissible = false,
    onDismiss,
    icon,
    actions,
    class: className = ''
  }: Props = $props();
  
  const variantConfig: Record<Variant, { 
    bg: string; 
    border: string; 
    text: string; 
    iconBg: string;
    defaultIcon: string;
  }> = {
    error: {
      bg: 'bg-red-950/50',
      border: 'border-red-500/30',
      text: 'text-red-100',
      iconBg: 'bg-red-500/20',
      defaultIcon: 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
    },
    warning: {
      bg: 'bg-orange-950/50',
      border: 'border-orange-500/30',
      text: 'text-orange-100',
      iconBg: 'bg-orange-500/20',
      defaultIcon: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z'
    },
    info: {
      bg: 'bg-blue-950/50',
      border: 'border-blue-500/30',
      text: 'text-blue-100',
      iconBg: 'bg-blue-500/20',
      defaultIcon: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
    },
    success: {
      bg: 'bg-green-950/50',
      border: 'border-green-500/30',
      text: 'text-green-100',
      iconBg: 'bg-green-500/20',
      defaultIcon: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'
    }
  };
  
  const config = variantConfig[variant];
</script>

<div class="rounded-xl border {config.bg} {config.border} p-4 {className} backdrop-blur-sm shadow-lg transition-all duration-300">
  <div class="flex gap-4">
    <!-- Icon -->
    <div class="flex-shrink-0">
      <div class="w-10 h-10 rounded-lg {config.iconBg} flex items-center justify-center">
        {#if icon}
          {@render icon()}
        {:else}
          <svg class="w-6 h-6 {config.text}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="{config.defaultIcon}" />
          </svg>
        {/if}
      </div>
    </div>
    
    <!-- Content -->
    <div class="flex-1 min-w-0">
      {#if title}
        <h3 class="text-base font-semibold {config.text} mb-1">
          {title}
        </h3>
      {/if}
      <p class="text-sm {config.text} opacity-90">
        {message}
      </p>
      
      {#if actions}
        <div class="mt-3 flex gap-2">
          {@render actions()}
        </div>
      {/if}
    </div>
    
    <!-- Dismiss Button -->
    {#if dismissible}
      <button
        onclick={onDismiss}
        class="flex-shrink-0 w-6 h-6 rounded-lg {config.text} hover:bg-white/10 transition-colors flex items-center justify-center"
        aria-label="Cerrar"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    {/if}
  </div>
</div>