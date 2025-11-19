<script lang="ts">
  import { onMount } from "svelte";
  import { load } from "@tauri-apps/plugin-store";
  import { setApiToken, getLights } from "$lib/lifx";
  import Setup from "../components/Setup.svelte";
  import Dashboard from "../components/Dashboard.svelte";

  let isAuthenticated = $state(false);
  let checking = $state(true);

  onMount(async () => {
    // We no longer load the token from the store here.
    // The backend handles the token via keyring.
    // However, we might want to check if a token exists to show the setup screen.
    // For now, let's assume if getLights fails, we need a token.
    try {
      await getLights();
      // If getLights succeeds, we are authenticated.
      isAuthenticated = true;
    } catch (e) {
      console.error("Failed to get lights, likely no token:", e);
      // If we fail, we might need to show setup.
      // The UI currently shows Setup if isAuthenticated is false.
      isAuthenticated = false;
    } finally {
      checking = false;
    }
  });

  function handleTokenSet() {
    isAuthenticated = true;
  }
</script>

<div
  class="min-h-screen bg-surface text-white font-sans selection:bg-purple-500/30"
>
  <!-- Background Gradients Removed -->

  <main class="relative z-10 h-screen max-w-7xl mx-auto">
    {#if checking}
      <div class="h-full flex items-center justify-center">
        <div
          class="w-8 h-8 border-2 border-purple-500 border-t-transparent rounded-full animate-spin"
        ></div>
      </div>
    {:else if isAuthenticated}
      <Dashboard />
    {:else}
      <Setup onTokenSet={handleTokenSet} />
    {/if}
  </main>
</div>
