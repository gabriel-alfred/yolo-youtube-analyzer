<script lang="ts">
  import Button from './Button.svelte';
  
  type Variant = 'info' | 'warning' | 'danger' | 'success';
  
  interface Props {
    open?: boolean;
    variant?: Variant;
    title: string;
    message: string;
    icon?: import('svelte').Snippet;
    children?: import('svelte').Snippet;
    cancelText?: string;
    confirmText?: string;
    onCancel?: () => void;
    onClose?: () => void;
    class?: string;
  }
  
  let {
    open = $bindable(false),
    variant = 'info',
    title,
    message,
    icon,
    children,
    cancelText = 'Cancelar',
    confirmText = 'Aceptar',
    onCancel,
    onClose,
    class: className = ''
  }: Props = $props();
  
  const variantConfig: Record<Variant, {
    iconBg: string;
    iconColor: string;
    defaultIcon: string;
  }> = {
    info: {
      iconBg: 'bg-blue-500/20',
      iconColor: 'text-blue-400',
      defaultIcon: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
    },
    warning: {
      iconBg: 'bg-orange-500/20',
      iconColor: 'text-orange-400',
      defaultIcon: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z'
    },
    danger: {
      iconBg: 'bg-red-500/20',
      iconColor: 'text-red-400',
      defaultIcon: 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
    },
    success: {
      iconBg: 'bg-green-500/20',
      iconColor: 'text-green-400',
      defaultIcon: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'
    }
  };
  
  const config = variantConfig[variant];
  
  function handleClose() {
    open = false;
    onClose?.();
  }
  
  function handleCancel() {
    open = false;
    onCancel?.();
  }
  
  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      handleClose();
    }
  }
  
  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      handleClose();
    }
  }
</script>

{#if open}
  <!-- Backdrop -->
  <div 
    role="dialog"
    aria-modal="true"
    aria-labelledby="dialog-title"
    aria-describedby="dialog-description"
    class="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-200"
    onclick={handleBackdropClick}
    onkeydown={handleKeyDown}
    tabindex="-1"
  >
    <!-- Dialog -->
    <div class="bg-slate-900/95 border border-red-500/30 rounded-xl shadow-2xl shadow-red-500/20 max-w-md w-full animate-in zoom-in-95 duration-200 {className}">
      <!-- Header with Icon -->
      <div class="flex items-start gap-4 p-6 pb-4">
        <!-- Icon -->
        <div class="flex-shrink-0">
          <div class="w-12 h-12 rounded-xl {config.iconBg} flex items-center justify-center" aria-hidden="true">
            {#if icon}
              {@render icon()}
            {:else}
              <svg class="w-6 h-6 {config.iconColor}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="{config.defaultIcon}" />
              </svg>
            {/if}
          </div>
        </div>
        
        <!-- Title and Close -->
        <div class="flex-1">
          <h2 id="dialog-title" class="text-xl font-bold text-red-100 mb-2">
            {title}
          </h2>
          <p id="dialog-description" class="text-sm text-red-100/70 leading-relaxed">
            {message}
          </p>
        </div>
        
        <!-- Close Button -->
        <button
          type="button"
          onclick={handleClose}
          class="flex-shrink-0 w-8 h-8 rounded-lg text-red-300 hover:text-red-100 hover:bg-red-500/10 transition-colors flex items-center justify-center"
          aria-label="Cerrar diálogo"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <!-- Custom Content -->
      {#if children}
        <div class="px-6 py-4 border-t border-red-500/20">
          {@render children()}
        </div>
      {/if}
      
      <!-- Footer Actions -->
      <div class="flex justify-end gap-3 p-6 pt-4 border-t border-red-500/20 bg-slate-900/50">
        {#if onCancel}
          <Button 
            variant="ghost" 
            size="md"
            onclick={handleCancel}
          >
            {cancelText}
          </Button>
        {/if}
        <Button 
          variant={variant === 'danger' ? 'danger' : 'primary'}
          size="md"
          onclick={handleClose}
        >
          {confirmText}
        </Button>
      </div>
    </div>
  </div>
{/if}