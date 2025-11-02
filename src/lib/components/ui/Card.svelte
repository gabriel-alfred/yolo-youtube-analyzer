<script lang="ts">
  type Variant = 'default' | 'gradient' | 'glass' | 'elevated';
  
  interface Props {
    variant?: Variant;
    hoverable?: boolean;
    padding?: 'none' | 'sm' | 'md' | 'lg';
    header?: import('svelte').Snippet;
    footer?: import('svelte').Snippet;
    children: import('svelte').Snippet;
    class?: string;
    allowOverflow?: boolean; // Nueva prop para controlar el overflow
  }
  
  let {
    variant = 'default',
    hoverable = false,
    padding = 'md',
    header,
    footer,
    children,
    class: className = '',
    allowOverflow = false
  }: Props = $props();
  
  const baseClasses = 'rounded-xl transition-all duration-300';
  
  const variantClasses: Record<Variant, string> = {
    default: 'bg-slate-900/70 border border-red-500/20',
    gradient: 'bg-gradient-to-br from-slate-900/90 via-red-950/40 to-slate-900/90 border border-red-500/30',
    glass: 'backdrop-blur-xl bg-slate-900/40 border border-red-500/20 shadow-xl',
    elevated: 'bg-slate-900/80 border border-red-500/30 shadow-2xl shadow-red-500/10'
  };
  
  const hoverClasses = hoverable 
    ? 'hover:border-red-500/50 hover:shadow-xl hover:shadow-red-500/10 hover:-translate-y-1 cursor-pointer' 
    : '';
  
  const overflowClass = allowOverflow ? 'overflow-visible' : 'overflow-hidden';
  
  const paddingClasses: Record<typeof padding, string> = {
    none: '',
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8'
  };
  
  const combinedClasses = `${baseClasses} ${variantClasses[variant]} ${hoverClasses} ${overflowClass} ${className}`;
  const contentPadding = header || footer ? '' : paddingClasses[padding];
</script>

<div class={combinedClasses}>
  <!-- Gradient Overlay for Gradient Variant -->
  {#if variant === 'gradient'}
    <div class="absolute inset-0 bg-gradient-to-br from-red-500/5 via-transparent to-orange-500/5 pointer-events-none"></div>
  {/if}
  
  <!-- Header -->
  {#if header}
    <div class="px-6 py-4 border-b border-red-500/20 bg-slate-900/30">
      {@render header()}
    </div>
  {/if}
  
  <!-- Content -->
  <div class="{header || footer ? paddingClasses[padding] : contentPadding} relative">
    {@render children()}
  </div>
  
  <!-- Footer -->
  {#if footer}
    <div class="px-6 py-4 border-t border-red-500/20 bg-slate-900/30">
      {@render footer()}
    </div>
  {/if}
  
  <!-- Hover Glow Effect -->
  {#if hoverable}
    <div class="absolute inset-0 opacity-0 group-hover:opacity-100 bg-gradient-to-br from-red-500/5 to-orange-500/5 pointer-events-none transition-opacity duration-300"></div>
  {/if}
</div>