<script lang="ts">
    import type { Light } from "$lib/lifx";
    import { setLightState, toggleLight } from "$lib/lifx";
    import { goto } from "$app/navigation";
    import { fade, fly } from 'svelte/transition';

    let { light } = $props<{ light: Light }>();
    
    // Local state for reactive UI
    let localPower = $state(light.power);
    let localBrightness = $state(light.brightness);
    let localHue = $state(light.color.hue);
    let localSaturation = $state(light.color.saturation);
    let localKelvin = $state(light.color.kelvin);
    let activeTab = $state(light.color.saturation === 0 ? 'white' : 'color');
    let updating = $state(false);
    
    // Debounce timer
    let debounceTimer: number | null = null;

    async function handleTogglePower() {
        if (updating || !light.connected) return;
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

    function debounceUpdate(updateFn: () => Promise<void>) {
        if (debounceTimer !== null) {
            clearTimeout(debounceTimer);
        }
        debounceTimer = setTimeout(async () => {
            try {
                await updateFn();
            } catch (e) {
                console.error(e);
            }
        }, 300) as unknown as number;
    }

    function handleBrightnessChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const value = parseFloat(target.value);
        localBrightness = value;
        
        debounceUpdate(() => 
            setLightState(light.id, { brightness: value, duration: 0.5 })
        );
    }

    function handleHueChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const value = parseFloat(target.value);
        localHue = value;
        
        debounceUpdate(() => 
            setLightState(light.id, { 
                color: `hue:${value} saturation:${localSaturation} brightness:${localBrightness} kelvin:${localKelvin}`,
                duration: 0.5 
            })
        );
    }

    function handleSaturationChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const value = parseFloat(target.value);
        localSaturation = value;
        
        debounceUpdate(() => 
            setLightState(light.id, { 
                color: `hue:${localHue} saturation:${value} brightness:${localBrightness} kelvin:${localKelvin}`,
                duration: 0.5 
            })
        );
    }

    function handleKelvinChange(e: Event) {
        const target = e.target as HTMLInputElement;
        const value = parseInt(target.value);
        localKelvin = value;
        
        debounceUpdate(() => 
            setLightState(light.id, { 
                color: `hue:${localHue} saturation:0 brightness:${localBrightness} kelvin:${value}`,
                duration: 0.5 
            })
        );
    }

    function setTab(tab: 'color' | 'white') {
        activeTab = tab;
        if (tab === 'white') {
            // When switching to white, set saturation to 0 immediately for preview
            localSaturation = 0;
            debounceUpdate(() => 
                setLightState(light.id, { 
                    color: `kelvin:${localKelvin} saturation:0`,
                    duration: 1.0 
                })
            );
        } else {
            // When switching to color, restore saturation if it was 0
            if (localSaturation === 0) localSaturation = 1.0;
             debounceUpdate(() => 
                setLightState(light.id, { 
                    color: `hue:${localHue} saturation:${localSaturation}`,
                    duration: 1.0 
                })
            );
        }
    }

    // Compute color preview
    let previewColor = $derived(
        activeTab === 'color' 
            ? `hsl(${localHue}, ${localSaturation * 100}%, 50%)`
            : `hsl(${kelvinToHue(localKelvin)}, 100%, 80%)` // Approximate warm/cool tint
    );

    // Helper to approximate Kelvin to Hue for UI feedback
    function kelvinToHue(k: number) {
        // Very rough approximation for UI visuals only
        if (k < 3000) return 30; // Orange
        if (k < 5000) return 45; // Yellow-ish
        return 200; // Blue-ish
    }
    
    function handleBack() {
        goto('/');
    }
</script>

<div class="relative h-screen w-full flex flex-col bg-[#121212] overflow-hidden">
    <!-- Dynamic Background -->
    <div 
        class="fixed inset-0 transition-colors duration-1000 ease-in-out opacity-20 pointer-events-none bg-[#121212]"
        style="background: radial-gradient(circle at 50% 30%, {previewColor}, #121212 80%);"
    ></div>

    <!-- Header -->
    <header class="absolute top-0 left-0 right-0 z-50 flex items-center justify-between px-6 py-6 shrink-0 bg-gradient-to-b from-[#121212]/60 via-[#121212]/30 to-transparent pointer-events-none">
        <button
            onclick={handleBack}
            class="p-2 rounded-full bg-white/5 hover:bg-white/10 backdrop-blur-md border border-white/10 text-white transition-all active:scale-95 pointer-events-auto"
            aria-label="Back"
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
        </button>
        
        <div class="text-center pointer-events-auto">
            <h1 class="text-base font-semibold text-white tracking-wide">{light.label}</h1>
            <div class="flex items-center justify-center gap-2 text-[10px] font-medium text-white/40 uppercase tracking-wider">
                <span class="w-1.5 h-1.5 rounded-full" class:bg-green-500={localPower === 'on'} class:bg-red-500={localPower === 'off'}></span>
                {localPower === 'on' ? 'Online' : 'Offline'}
            </div>
        </div>

        <div class="w-9"></div> <!-- Spacer for balance -->
    </header>

    <!-- Main Content Wrapper -->
    <div class="flex-1 flex flex-col md:flex-row relative z-10 h-full">
        
        <!-- Left Column: Visuals (Orb) -->
        <div class="flex-1 flex items-center justify-center p-6 md:p-12 relative">
             <!-- Hero Orb -->
            <div class="relative group shrink-0">
                <div 
                    class="w-48 h-48 md:w-64 md:h-64 rounded-full transition-all duration-500 ease-out shadow-[0_0_60px_rgba(0,0,0,0.5)]"
                    style="
                        background: {previewColor};
                        box-shadow: 0 0 {localBrightness * (window.innerWidth > 768 ? 100 : 60)}px {previewColor};
                        opacity: {localPower === 'on' ? 0.9 + (localBrightness * 0.1) : 0.1};
                        transform: scale({localPower === 'on' ? 1 : 0.9});
                    "
                ></div>
                
                <!-- Power Button Overlay -->
                <button
                    onclick={handleTogglePower}
                    disabled={!light.connected}
                    class="absolute bottom-0 left-1/2 -translate-x-1/2 translate-y-1/2 w-14 h-14 md:w-16 md:h-16 rounded-full bg-[#1a1a1a] border-4 border-[#121212] flex items-center justify-center text-white shadow-xl transition-transform active:scale-95 hover:bg-[#252525] z-20 disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100"
                    aria-label="Toggle Power"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class:text-white={localPower === 'on'} class:text-gray-600={localPower === 'off'} class="md:w-8 md:h-8">
                        <path d="M18.36 6.64a9 9 0 1 1-12.73 0"></path>
                        <line x1="12" y1="2" x2="12" y2="12"></line>
                    </svg>
                </button>
            </div>
        </div>

        <!-- Right Column: Controls -->
        <div class="flex-1 flex items-center justify-center p-6 md:p-12 md:pt-24 overflow-y-auto md:overflow-visible">
            <div class="w-full max-w-sm space-y-8 md:space-y-10 transition-opacity duration-300" class:opacity-50={!light.connected} class:pointer-events-none={!light.connected}>
                
                <!-- Brightness Slider (Always Visible) -->
                <div class="space-y-2">
                    <div class="flex justify-between text-xs font-medium text-white/50 uppercase tracking-wider">
                        <span>Brightness</span>
                        <span>{Math.round(localBrightness * 100)}%</span>
                    </div>
                    <div class="relative h-10 md:h-12 bg-white/5 rounded-xl overflow-hidden border border-white/10 group">
                        <div 
                            class="absolute inset-y-0 left-0 bg-white/20 transition-all duration-100"
                            style="width: {localBrightness * 100}%"
                        ></div>
                        <input
                            type="range"
                            min="0"
                            max="1"
                            step="0.01"
                            value={localBrightness}
                            oninput={handleBrightnessChange}
                            disabled={!light.connected}
                            class="absolute inset-0 w-full h-full opacity-0 cursor-pointer disabled:cursor-not-allowed"
                        />
                        <!-- Icon overlay -->
                        <div class="absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white/50"><circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/></svg>
                        </div>
                    </div>
                </div>

                <!-- Mode Tabs -->
                <div class="bg-black/40 p-1 rounded-xl flex relative">
                    <button 
                        class="flex-1 py-1.5 md:py-2 text-xs md:text-sm font-medium rounded-lg transition-all relative z-10"
                        class:text-black={activeTab === 'color'}
                        class:text-white={activeTab !== 'color'}
                        onclick={() => setTab('color')}
                        disabled={!light.connected}
                    >
                        Color
                    </button>
                    <button 
                        class="flex-1 py-1.5 md:py-2 text-xs md:text-sm font-medium rounded-lg transition-all relative z-10"
                        class:text-black={activeTab === 'white'}
                        class:text-white={activeTab !== 'white'}
                        onclick={() => setTab('white')}
                        disabled={!light.connected}
                    >
                        White
                    </button>
                    
                    <!-- Sliding Background -->
                    <div 
                        class="absolute top-1 bottom-1 rounded-lg bg-white transition-all duration-300 ease-out shadow-sm"
                        style="
                            left: {activeTab === 'color' ? '4px' : '50%'}; 
                            width: calc(50% - 4px);
                        "
                    ></div>
                </div>

                <!-- Tab Content -->
                <div class="min-h-[160px] md:min-h-[180px]">
                    {#if activeTab === 'color'}
                        <div in:fade={{ duration: 200 }} class="space-y-6 md:space-y-8">
                            <!-- Hue -->
                            <div class="space-y-2">
                                <div class="flex justify-between text-xs font-medium text-white/50 uppercase tracking-wider">
                                    <span>Hue</span>
                                    <span>{Math.round(localHue)}°</span>
                                </div>
                                <div class="relative h-10 md:h-12 rounded-xl overflow-hidden border border-white/10">
                                    <div class="absolute inset-0" style="background: linear-gradient(to right, #ff0000, #ffff00, #00ff00, #00ffff, #0000ff, #ff00ff, #ff0000);"></div>
                                    <input
                                        type="range"
                                        min="0"
                                        max="360"
                                        step="1"
                                        value={localHue}
                                        oninput={handleHueChange}
                                        disabled={!light.connected}
                                        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer disabled:cursor-not-allowed"
                                    />
                                    <!-- Thumb Indicator (Visual only) -->
                                    <div 
                                        class="absolute top-1 bottom-1 w-1.5 bg-white rounded-full shadow-lg pointer-events-none transition-all duration-75"
                                        style="left: {(localHue / 360) * 100}%; transform: translateX(-50%);"
                                    ></div>
                                </div>
                            </div>

                            <!-- Saturation -->
                            <div class="space-y-2">
                                <div class="flex justify-between text-xs font-medium text-white/50 uppercase tracking-wider">
                                    <span>Saturation</span>
                                    <span>{Math.round(localSaturation * 100)}%</span>
                                </div>
                                <div class="relative h-10 md:h-12 rounded-xl overflow-hidden border border-white/10 bg-white/5">
                                    <div 
                                        class="absolute inset-0 opacity-50"
                                        style="background: linear-gradient(to right, #ffffff, hsl({localHue}, 100%, 50%));"
                                    ></div>
                                    <input
                                        type="range"
                                        min="0"
                                        max="1"
                                        step="0.01"
                                        value={localSaturation}
                                        oninput={handleSaturationChange}
                                        disabled={!light.connected}
                                        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer disabled:cursor-not-allowed"
                                    />
                                     <!-- Thumb Indicator -->
                                     <div 
                                        class="absolute top-1 bottom-1 w-1.5 bg-white rounded-full shadow-lg pointer-events-none transition-all duration-75"
                                        style="left: {localSaturation * 100}%; transform: translateX(-50%);"
                                    ></div>
                                </div>
                            </div>
                        </div>
                    {:else}
                        <div in:fade={{ duration: 200 }} class="space-y-6 md:space-y-8">
                            <!-- Kelvin -->
                            <div class="space-y-2">
                                <div class="flex justify-between text-xs font-medium text-white/50 uppercase tracking-wider">
                                    <span>Temperature</span>
                                    <span>{localKelvin}K</span>
                                </div>
                                <div class="relative h-10 md:h-12 rounded-xl overflow-hidden border border-white/10">
                                    <div class="absolute inset-0" style="background: linear-gradient(to right, #ff9329, #fffaf4, #d4e6ff);"></div>
                                    <input
                                        type="range"
                                        min="2500"
                                        max="9000"
                                        step="100"
                                        value={localKelvin}
                                        oninput={handleKelvinChange}
                                        disabled={!light.connected}
                                        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer disabled:cursor-not-allowed"
                                    />
                                    <!-- Thumb Indicator -->
                                    <div 
                                        class="absolute top-1 bottom-1 w-1.5 bg-white/80 rounded-full shadow-lg pointer-events-none transition-all duration-75 backdrop-blur-sm border border-black/10"
                                        style="left: {((localKelvin - 2500) / (9000 - 2500)) * 100}%; transform: translateX(-50%);"
                                    ></div>
                                </div>
                                <div class="flex justify-between text-[10px] text-white/30 font-medium uppercase tracking-wider px-1">
                                    <span>Warm</span>
                                    <span>Cool</span>
                                </div>
                            </div>
                        </div>
                    {/if}
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    /* Hide scrollbar for Chrome, Safari and Opera */
    .custom-scrollbar::-webkit-scrollbar {
        display: none;
    }

    /* Hide scrollbar for IE, Edge and Firefox */
    .custom-scrollbar {
        -ms-overflow-style: none;  /* IE and Edge */
        scrollbar-width: none;  /* Firefox */
    }
</style>
