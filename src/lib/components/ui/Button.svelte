<script lang="ts">
  type Variant = 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
  type Size = 'sm' | 'md' | 'lg';
  
  interface Props {
    variant?: Variant;
    size?: Size;
    disabled?: boolean;
    fullWidth?: boolean;
    loading?: boolean;
    icon?: import('svelte').Snippet;
    children?: import('svelte').Snippet;
    onclick?: (e: MouseEvent) => void;
    type?: 'button' | 'submit' | 'reset';
    class?: string;
  }
  
  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    fullWidth = false,
    loading = false,
    icon,
    children,
    onclick,
    type = 'button',
    class: className = ''
  }: Props = $props();
  
  const baseClasses = 'relative font-semibold rounded-lg transition-all duration-300 overflow-hidden group disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring-2 focus:ring-red-500/50';
  
  const variantClasses: Record<Variant, string> = {
    primary: 'text-white shadow-lg shadow-red-500/20 hover:shadow-red-500/40',
    secondary: 'bg-slate-800 text-red-100 hover:bg-slate-700 border border-red-500/20',
    outline: 'bg-transparent text-red-300 border-2 border-red-500/50 hover:bg-red-500/10 hover:border-red-500',
    ghost: 'bg-transparent text-red-300 hover:bg-red-500/10',
    danger: 'text-white shadow-lg shadow-red-600/20 hover:shadow-red-600/40'
  };
  
  const sizeClasses: Record<Size, string> = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  };
  
  const widthClass = fullWidth ? 'w-full' : '';
  
  const combinedClasses = `${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${widthClass} ${className}`;
</script>

<button
  {type}
  class={combinedClasses}
  {disabled}
  onclick={onclick}
>
  <!-- Gradient Background for Primary/Danger -->
  {#if variant === 'primary'}
    <div class="absolute inset-0 bg-gradient-to-r from-red-600 to-orange-600 group-hover:from-red-500 group-hover:to-orange-500 transition-all"></div>
    <div class="absolute inset-0 bg-gradient-to-r from-red-600 via-orange-600 to-red-600 opacity-0 group-hover:opacity-100 blur-xl transition-opacity"></div>
  {/if}
  
  {#if variant === 'danger'}
    <div class="absolute inset-0 bg-gradient-to-r from-red-700 to-red-600 group-hover:from-red-600 group-hover:to-red-500 transition-all"></div>
    <div class="absolute inset-0 bg-gradient-to-r from-red-700 via-red-600 to-red-700 opacity-0 group-hover:opacity-100 blur-xl transition-opacity"></div>
  {/if}
  
  <!-- Content -->
  <span class="relative z-10 flex items-center justify-center gap-2">
    {#if loading}
      <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
    {:else if icon}
      {@render icon?.()}
    {/if}
    {#if children}
      {@render children?.()}
    {/if}
  </span>
</button>