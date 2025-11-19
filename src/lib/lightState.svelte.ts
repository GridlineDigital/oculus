import { getLights, getScenes, type Light, type Scene } from './lifx';

class LightState {
    lights = $state<Light[]>([]);
    scenes = $state<Scene[]>([]);
    lastUpdated = $state<number>(0);
    loading = $state(false);
    error = $state<string | null>(null);

    async refresh() {
        // Only show loading state if we have no data
        // Otherwise, fetch silently in the background
        const shouldShowLoading = this.lights.length === 0;
        
        if (shouldShowLoading) {
            this.loading = true;
        }
        
        this.error = null;
        try {
            const [lightsData, scenesData] = await Promise.all([
                getLights(),
                getScenes()
            ]);
            this.lights = lightsData;
            this.scenes = scenesData;
            this.lastUpdated = Date.now();
        } catch (e) {
            console.error("Failed to refresh data:", e);
            this.error = "Failed to load data";
        } finally {
            if (shouldShowLoading) {
                this.loading = false;
            }
        }
    }

    getLight(id: string): Light | undefined {
        return this.lights.find(l => l.id === id);
    }

    // Optimistic update helper
    updateLight(id: string, updates: Partial<Light>) {
        const index = this.lights.findIndex(l => l.id === id);
        if (index !== -1) {
            this.lights[index] = { ...this.lights[index], ...updates };
        }
    }
}

export const lightState = new LightState();
