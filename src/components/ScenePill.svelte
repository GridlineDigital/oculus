<script lang="ts">
    import type { Scene } from "$lib/lifx";
    import { activateScene } from "$lib/lifx";

    let { scene } = $props<{ scene: Scene }>();
    let activating = $state(false);

    async function handleActivate() {
        if (activating) return;
        activating = true;
        try {
            await activateScene(scene.uuid);
        } catch (e) {
            console.error(e);
        } finally {
            setTimeout(() => {
                activating = false;
            }, 1000);
        }
    }
</script>

<button
    onclick={handleActivate}
    disabled={activating}
    class="group relative px-5 py-2.5 rounded-xl bg-glass border border-border-glass hover:bg-glass-hover hover:border-border-glass-hover transition-all duration-300 active:scale-95 flex items-center gap-3 overflow-hidden {activating ? 'ring-1 ring-border-border-glass-focus' : ''}"
>
    <!-- Subtle Gradient Background on Hover -->
    <div class="absolute inset-0 bg-gradient-to-r from-purple-500/10 to-blue-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>

    <!-- Icon/Indicator -->
    {#if activating}
        <div class="relative w-2.5 h-2.5">
            <div class="absolute inset-0 rounded-full border-2 border-border-glass-focus border-t-white animate-spin"></div>
        </div>
    {/if}
    
    <span class="relative text-sm font-medium text-white/80 group-hover:text-white transition-colors">
        {scene.name}
    </span>
</button>
