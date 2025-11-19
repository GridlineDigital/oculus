# API Documentation

This document details the Tauri commands available to the frontend and the shared data types.

## Tauri Commands

These commands are invoked from the frontend using `invoke('command_name', { args })`.

### `set_api_token`

Sets the LIFX API token for the session.

- **Arguments**:
  - `token` (string): The personal access token from LIFX.
- **Returns**: `void`

### `get_lights`

Fetches a list of all lights associated with the account.

- **Arguments**: None
- **Returns**: `Promise<Light[]>`

### `set_light_state`

Sets the state (color, power, brightness) of a light or group.

- **Arguments**:
  - `selector` (string): The LIFX selector (e.g., `id:12345`, `all`).
  - `payload` (LifxStatePayload): The state changes to apply.
- **Returns**: `Promise<void>`

### `toggle_light`

Toggles the power status of a light.

- **Arguments**:
  - `selector` (string): The LIFX selector.
- **Returns**: `Promise<void>`

### `get_scenes`

Fetches a list of all scenes associated with the account.

- **Arguments**: None
- **Returns**: `Promise<Scene[]>`

### `activate_scene`

Activates a specific scene.

- **Arguments**:
  - `uuid` (string): The UUID of the scene to activate.
- **Returns**: `Promise<void>`

## Data Types

### `Light`

Represents a LIFX light device.

```typescript
interface Light {
  id: string;
  label: string;
  power: "on" | "off";
  brightness: number;
  color: Color;
}
```

### `Color`

Represents the color state of a light.

```typescript
interface Color {
  hue: number;
  saturation: number;
  kelvin: number;
}
```

### `Scene`

Represents a saved configuration of lights.

```typescript
interface Scene {
  uuid: string;
  name: string;
  account: Account;
}
```

### `LifxStatePayload`

Payload for updating light state.

```typescript
interface LifxStatePayload {
  power?: "on" | "off";
  color?: string;
  brightness?: number;
  duration?: number;
}
```
