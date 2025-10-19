<script lang="ts">
  type InputType = 'text' | 'email' | 'password' | 'number' | 'url' | 'search' | 'tel';
  
  interface Props {
    value?: string | number;
    type?: InputType;
    placeholder?: string;
    label?: string;
    error?: string;
    helperText?: string;
    disabled?: boolean;
    required?: boolean;
    fullWidth?: boolean;
    icon?: import('svelte').Snippet;
    rightIcon?: import('svelte').Snippet;
    oninput?: (e: Event) => void;
    onchange?: (e: Event) => void;
    class?: string;
  }
  
  let {
    value = $bindable(''),
    type = 'text',
    placeholder = '',
    label = '',
    error = '',
    helperText = '',
    disabled = false,
    required = false,
    fullWidth = false,
    icon,
    rightIcon,
    oninput,
    onchange,
    class: className = ''
  }: Props = $props();
  
  let focused = $state(false);
  let inputId = `input-${Math.random().toString(36).substr(2, 9)}`;
  
  const baseClasses = 'w-full px-4 py-2.5 bg-slate-900/60 border rounded-lg transition-all duration-300 text-red-50 placeholder:text-red-300/40 focus:outline-none focus:ring-0 outline-none disabled:opacity-50 disabled:cursor-not-allowed';
  
  const getCombinedClasses = () => {
    const borderClasses = error 
      ? 'border-red-500 focus:border-red-400' 
      : 'border-red-500/30 focus:border-red-500 hover:border-red-500/50';
    const shadowClasses = focused && !error ? 'shadow-lg shadow-red-500/10' : '';
    const paddingClasses = icon ? 'pl-11' : rightIcon ? 'pr-11' : '';
    
    return `${baseClasses} ${borderClasses} ${shadowClasses} ${paddingClasses} ${className}`;
  };
</script>

<style>
  input {
    outline: none !important;
  }
  
  input:focus {
    outline: none !important;
    box-shadow: none !important;
  }
  
  input:active {
    outline: none !important;
  }
</style>

<div class="{fullWidth ? 'w-full' : ''} {className}">
  {#if label}
    <label for={inputId} class="block text-sm font-medium text-red-200 mb-2">
      {label}
      {#if required}
        <span class="text-red-400">*</span>
      {/if}
    </label>
  {/if}
  
  <div class="relative">
    <!-- Left Icon -->
    {#if icon}
      <div class="absolute left-3 top-1/2 -translate-y-1/2 text-red-400/60 pointer-events-none">
        {@render icon()}
      </div>
    {/if}
    
    <!-- Input -->
    <input
      id={inputId}
      {type}
      {placeholder}
      {disabled}
      {required}
      bind:value
      class={getCombinedClasses()}
      onfocus={() => focused = true}
      onblur={() => focused = false}
      oninput={oninput}
      onchange={onchange}
    />
    
    <!-- Right Icon -->
    {#if rightIcon}
      <div class="absolute right-3 top-1/2 -translate-y-1/2 text-red-400/60 pointer-events-none">
        {@render rightIcon()}
      </div>
    {/if}
  </div>
  
  <!-- Helper Text or Error -->
  {#if error}
    <p class="mt-1.5 text-sm text-red-400 flex items-center gap-1">
      <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
      </svg>
      {error}
    </p>
  {:else if helperText}
    <p class="mt-1.5 text-sm text-red-300/60">{helperText}</p>
  {/if}
</div>