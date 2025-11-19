<script lang="ts">
    import type { Light } from "$lib/lifx";
    import { toggleLight } from "$lib/lifx";
    import { goto } from "$app/navigation";

    let { light } = $props<{ light: Light }>();
    let localPower = $state(light.power);
    let localBrightness = $state(light.brightness);
    let updating = $state(false);

    // Derived color for the Mini Orb
    let previewColor = $derived.by(() => {
        if (!light.connected) return '#9ca3af'; // gray-400 for offline
        const { hue, saturation, kelvin } = light.color;
        if (saturation === 0) {
            // Kelvin approximation
            if (kelvin < 3000) return '#ffb16d'; // Warm
            if (kelvin < 4500) return '#ffdbba'; // Neutral
            return '#cce0ff'; // Cool
        }
        return `hsl(${hue}, ${saturation * 100}%, 50%)`;
    });

    async function handleToggle(e: Event) {
        e.stopPropagation();
        if (updating) return;
        updating = true;
        try {
            await toggleLight(light.id);
            localPower = localPower === "on" ? "off" : "on";
        } catch (e) {
            console.error(e);
        } finally {
            updating = false;
        }
    }

    async function handleCardClick() {
        try {
            await goto(`/light/${light.id}`);
        } catch (e) {
            console.error("Navigation failed:", e);
        }
    }
</script>

<div
    role="button"
    tabindex="0"
    onclick={handleCardClick}
    onkeydown={(e) => e.key === 'Enter' && handleCardClick()}
    class="group relative overflow-hidden rounded-2xl bg-glass border border-border-glass p-5 transition-all duration-300 hover:bg-glass-hover hover:border-border-glass-hover hover:scale-[1.02] hover:shadow-xl cursor-pointer"
>
    <!-- Glow Effect on Hover -->
    <div 
        class="absolute -right-10 -top-10 w-32 h-32 bg-glass rounded-full blur-3xl transition-opacity duration-500 opacity-0 group-hover:opacity-100 pointer-events-none"
        style="background-color: {previewColor};"
    ></div>

    <div class="flex justify-between items-start relative z-10">
        <div class="flex items-center gap-4">
            <!-- Mini Orb -->
            <div class="relative w-12 h-12 shrink-0">
                <div 
                    class="w-full h-full rounded-full transition-all duration-500 ease-out shadow-inner"
                    style="
                        background: {previewColor};
                        opacity: {localPower === 'on' ? 1 : 0.2};
                        box-shadow: 0 0 {localPower === 'on' ? 20 : 0}px {previewColor};
                        transform: scale({localPower === 'on' ? 1 : 0.9});
                    "
                ></div>
                <!-- Online Status Dot -->
                <div class="absolute -bottom-1 -right-1 w-3 h-3 bg-surface rounded-full flex items-center justify-center">
                    {#if !light.connected}
                        <div class="w-1.5 h-1.5 rounded-full bg-red-500" title="Offline"></div>
                    {:else}
                        <div class="w-1.5 h-1.5 rounded-full" class:bg-green-500={localPower === 'on'} class:bg-neutral-500={localPower === 'off'}></div>
                    {/if}
                </div>
            </div>
            
            <div class="flex flex-col" class:opacity-50={!light.connected}>
                <h3 class="font-semibold text-white text-lg leading-tight tracking-tight">{light.label}</h3>
                <p class="text-xs font-medium text-text-tertiary uppercase tracking-wider mt-1">
                    {#if !light.connected}
                        <span class="text-red-400">Offline</span>
                    {:else}
                        {localPower === "on" ? `${Math.round(localBrightness * 100)}%` : "Off"}
                    {/if}
                </p>
            </div>
        </div>

        <!-- Power Toggle -->
        <button
            onclick={handleToggle}
            disabled={updating || !light.connected}
            aria-label="Toggle light power"
            class="w-12 h-7 rounded-full relative transition-all duration-300 ease-out focus:outline-none focus:ring-2 focus:ring-glass-active {localPower === 'on' ? 'bg-white' : 'bg-glass-hover'} disabled:opacity-50 disabled:cursor-not-allowed"
        >
            <span
                class="absolute left-1 top-1 w-5 h-5 rounded-full transition-all duration-300 ease-out shadow-sm {localPower === 'on' ? 'bg-black translate-x-5' : 'bg-white/50'}"
            ></span>
        </button>
    </div>
</div>
