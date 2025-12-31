<script lang="ts">
  import Layout from "$lib/components/layout/Layout.svelte";
  import "../app.css";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import ConfirmationDialog from "$lib/components/ui/ConfirmationDialog.svelte";
  import { currentLanguage, t } from "$lib/i18n";

  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  let showExitDialog = $state(false);
  let isClosing = false; // Flag to prevent infinite loop

  // Reactive translations
  let translations = $derived($t);

  onMount(() => {
    // Initialize i18n
    currentLanguage.init();

    const appWindow = getCurrentWindow();

    // Intercept close request
    const unlistenPromise = appWindow.onCloseRequested(async (event) => {
      // If we are already in the process of closing (confirmed), let it happen.
      if (isClosing) {
        return;
      }

      // Prevent the immediate close to check for active analysis
      event.preventDefault();

      try {
        const activeSession = await invoke("get_active_analysis");
        if (activeSession) {
          showExitDialog = true;
        } else {
          // No analysis -> Just close
          isClosing = true;
          await appWindow.close();
        }
      } catch (e) {
        console.error("Error checking active analysis on exit:", e);
        // On error, force close to avoid locking the user
        isClosing = true;
        await appWindow.close();
      }
    });

    return () => {
      unlistenPromise.then((f) => f());
    };
  });

  async function confirmExit() {
    try {
      await invoke("stop_video_analysis");
    } catch (e) {
      console.error("Error stopping analysis on exit:", e);
    } finally {
      isClosing = true;
      const appWindow = getCurrentWindow();
      appWindow.close();
    }
  }
</script>

<Layout>
  {#snippet children()}
    {#if children}
      {@render children()}
    {/if}
  {/snippet}
</Layout>

<ConfirmationDialog
  bind:open={showExitDialog}
  title="Análisis en curso"
  message="Hay un análisis ejecutándose en segundo plano. ¿Deseas detenerlo y salir de la aplicación?"
  confirmText="Detener y Salir"
  variant="danger"
  onConfirm={confirmExit}
  cancelText="Cancelar"
/>
