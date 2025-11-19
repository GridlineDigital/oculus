import { invoke } from '@tauri-apps/api/core';
import { info, error } from '@tauri-apps/plugin-log';

export interface Light {
    id: string;
    label: string;
    power: string;
    brightness: number;
    color: {
        hue: number;
        saturation: number;
        kelvin: number;
    };
    connected: boolean;
}

export interface Scene {
    uuid: string;
    name: string;
    account: {
        uuid: string;
    };
}

export interface LifxStatePayload {
    power?: string;
    color?: string;
    brightness?: number;
    duration?: number;
}

export async function setApiToken(token: string): Promise<void> {
    try {
        await invoke('set_api_token', { token });
        await info('API token set');
    } catch (e) {
        await error(`Failed to set API token: ${e}`);
        throw e;
    }
}

export async function getLights(): Promise<Light[]> {
    try {
        const lights = await invoke<Light[]>('get_lights');
        await info(`Fetched ${lights.length} lights`);
        return lights;
    } catch (e) {
        await error(`Failed to get lights: ${e}`);
        throw e;
    }
}

export async function setLightState(selector: string, payload: LifxStatePayload): Promise<void> {
    try {
        await invoke('set_light_state', { selector, payload });
        await info(`Set state for ${selector}: ${JSON.stringify(payload)}`);
    } catch (e) {
        await error(`Failed to set light state: ${e}`);
        throw e;
    }
}

export async function toggleLight(selector: string): Promise<void> {
    try {
        await invoke('toggle_light', { selector });
        await info(`Toggled light ${selector}`);
    } catch (e) {
        await error(`Failed to toggle light: ${e}`);
        throw e;
    }
}

export async function getScenes(): Promise<Scene[]> {
    try {
        const scenes = await invoke<Scene[]>('get_scenes');
        await info(`Fetched ${scenes.length} scenes`);
        return scenes;
    } catch (e) {
        await error(`Failed to get scenes: ${e}`);
        throw e;
    }
}

export async function activateScene(uuid: string): Promise<void> {
    try {
        await invoke('activate_scene', { uuid });
        await info(`Activated scene ${uuid}`);
    } catch (e) {
        await error(`Failed to activate scene: ${e}`);
        throw e;
    }
}
