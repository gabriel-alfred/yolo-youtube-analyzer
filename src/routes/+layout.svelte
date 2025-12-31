<script lang="ts">
  import Layout from "$lib/components/layout/Layout.svelte";
  import "../app.css";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import ConfirmationDialog from "$lib/components/ui/ConfirmationDialog.svelte";

  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  let showExitDialog = $state(false);
  let isClosing = false; // Flag to prevent infinite loop

  onMount(() => {
    const appWindow = getCurrentWindow();

    // Intercept close request
    const unlisten = appWindow.listen(
      "tauri://close-requested",
      async (event) => {
        if (isClosing) return; // Allow close if we already confirmed

        try {
          const activeSession = await invoke("get_active_analysis");
          if (activeSession) {
            // Analysis active -> Prevent close and show dialog
            // event.preventDefault(); // Tauri v2 listener doesn't need preventDefault on the event object for this specific event type in explicit listener, but let's check docs logic.
            // Actually, 'tauri://close-requested' is an event we receive. To prevent close we usually don't need to do anything if we haven't called close() ourselves, BUT
            // Standard Tauri flow: user clicks X -> event emitted. If we don't close manually, it might wait?
            // Wait, usually we need to set `prevent_close` or similar in rust or just use the event.
            // In Tauri v2, typically we can just interception.

            showExitDialog = true;
          } else {
            // No analysis -> Just close
            isClosing = true;
            appWindow.close();
          }
        } catch (e) {
          console.error("Error checking active analysis on exit:", e);
          isClosing = true;
          appWindow.close();
        }
      },
    );

    return () => {
      unlisten.then((f) => f());
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
