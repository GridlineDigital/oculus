<script lang="ts">
    import { onMount } from "svelte";
    import { setLightState } from "$lib/lifx";
    import { lightState } from "$lib/lightState.svelte";
    import LightCard from "./LightCard.svelte";
    import ScenePill from "./ScenePill.svelte";
    import { fade } from 'svelte/transition';

    // Derived state for master toggle
    let anyLightsOn = $derived(lightState.lights.some(l => l.power === 'on'));

    async function fetchData() {
        // Trigger background refresh for lights and scenes
        lightState.refresh();
    }

    async function handleMasterToggle() {
        const targetPower = anyLightsOn ? 'off' : 'on';
        
        // Optimistic update
        lightState.lights.forEach(l => {
            if (l.connected) {
                lightState.updateLight(l.id, { power: targetPower });
            }
        });

        try {
            await setLightState('all', { power: targetPower, duration: 1.0 });
        } catch (e) {
            console.error("Master toggle failed:", e);
            lightState.refresh(); // Revert on error
        }
    }

    onMount(() => {
        fetchData();
        // Poll for updates every 5 seconds for snappier feel
        const interval = setInterval(() => lightState.refresh(), 5000);
        return () => clearInterval(interval);
    });
</script>

<div class="h-full w-full bg-surface text-white overflow-y-auto custom-scrollbar relative">
    
    <div class="max-w-7xl mx-auto px-6 pb-12">
        <!-- Header -->
        <header class="py-6 md:py-8 flex items-center justify-between">
            <div>
                <h1 class="text-2xl md:text-3xl font-bold tracking-tight">My Home</h1>
                <p class="text-text-tertiary text-sm font-medium mt-1">
                    {lightState.lights.length} Lights • {lightState.scenes.length} Scenes
                </p>
            </div>

            <div class="flex items-center gap-4">
                <!-- Master Switch -->
                <button
                    onclick={handleMasterToggle}
                    class="group flex items-center gap-3 px-4 py-2 rounded-full bg-glass hover:bg-glass-hover border border-border-glass transition-all active:scale-95"
                    aria-label={anyLightsOn ? "Turn all off" : "Turn all on"}
                >
                    <span class="text-sm font-medium text-text-secondary group-hover:text-white transition-colors hidden md:block">
                        {anyLightsOn ? 'All On' : 'All Off'}
                    </span>
                    <div 
                        class="w-10 h-6 rounded-full relative transition-colors duration-300 ease-in-out {anyLightsOn ? 'bg-white' : 'bg-glass-hover'}"
                    >
                        <div 
                            class="absolute top-1 left-1 w-4 h-4 rounded-full bg-surface transition-transform duration-300 shadow-sm"
                            class:translate-x-4={anyLightsOn}
                        ></div>
                    </div>
                </button>

                <!-- Refresh Button -->
                <button
                    onclick={fetchData}
                    class="p-2.5 rounded-full bg-glass hover:bg-glass-hover border border-border-glass text-text-secondary hover:text-white transition-all active:scale-95"
                    title="Refresh"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class:animate-spin={lightState.loading}
                    >
                        <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                        <path d="M3 3v5h5" />
                        <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
                        <path d="M16 16h5v5" />
                    </svg>
                </button>

                <!-- Settings Button -->
                <a
                    href="/settings"
                    class="p-2.5 rounded-full bg-glass hover:bg-glass-hover border border-border-glass text-text-secondary hover:text-white transition-all active:scale-95"
                    title="Settings"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.47a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
                </a>
            </div>
        </header>

        <!-- Content -->
        <div class="space-y-10">
            
            <!-- Error State -->
            {#if lightState.error}
                <div in:fade class="p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-200 flex items-center justify-between">
                    <span>{lightState.error}</span>
                    <button onclick={fetchData} class="text-sm font-bold hover:underline">Retry</button>
                </div>
            {/if}

            <!-- Scenes Section -->
            {#if lightState.scenes.length > 0}
                <section>
                    <h2 class="text-xs font-bold text-text-muted uppercase tracking-widest mb-4 ml-1">Scenes</h2>
                    <div class="flex flex-wrap gap-3">
                        {#each lightState.scenes as scene (scene.uuid)}
                            <ScenePill {scene} />
                        {/each}
                    </div>
                </section>
            {/if}

            <!-- Lights Section -->
            <section>
                <h2 class="text-xs font-bold text-text-muted uppercase tracking-widest mb-4 ml-1">Lights</h2>
                
                {#if lightState.loading && lightState.lights.length === 0}
                    <!-- Skeleton Loading -->
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                        {#each Array(4) as _}
                            <div class="h-40 rounded-2xl bg-glass border border-glass animate-pulse"></div>
                        {/each}
                    </div>
                {:else if lightState.lights.length > 0}
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                        {#each lightState.lights as light (light.id)}
                            <LightCard {light} />
                        {/each}
                    </div>
                {:else if !lightState.loading}
                    <div class="flex flex-col items-center justify-center py-20 text-text-muted">
                        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" class="mb-4 opacity-50"><path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"/><path d="M9 18h6"/><path d="M10 22h4"/></svg>
                        <p>No lights found.</p>
                    </div>
                {/if}
            </section>
        </div>
    </div>
</div>
