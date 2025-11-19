import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getLights } from '$lib/lifx';
import { lightState } from '$lib/lightState.svelte';

export const load: PageLoad = async ({ params }) => {
    // Try to get from global state first for instant navigation
    const cachedLight = lightState.getLight(params.id);
    if (cachedLight) {
        return { light: cachedLight };
    }

    // Fallback to fetching if not in state (e.g. deep link)
    const lights = await getLights();
    const light = lights.find(l => l.id === params.id);

    if (!light) {
        throw error(404, 'Light not found');
    }

    return {
        light
    };
};
