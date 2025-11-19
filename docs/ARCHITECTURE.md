# Architecture Overview

This document provides a high-level overview of the Light Effects application architecture.

## System Design

Light Effects is a desktop application built using [Tauri](https://tauri.app/), which combines a Rust backend with a web-based frontend.

### Tech Stack

- **Frontend**: SvelteKit (Svelte 5) + TypeScript + Tailwind CSS v4
- **Backend**: Rust (Tauri)
- **Build Tool**: Vite + Bun

### High-Level Diagram

```mermaid
graph TD
    User[User] --> UI[SvelteKit Frontend]
    UI -- IPC Commands --> Core[Tauri Core (Rust)]
    Core -- HTTP Requests --> LIFX[LIFX Cloud API]
    LIFX -- JSON Response --> Core
    Core -- Result/Error --> UI
```

## Frontend (SvelteKit)

The frontend is a Single Page Application (SPA) served by Tauri.

- **Components**: Located in `src/components/`.
  - `Dashboard.svelte`: The main control center with grid layout and global controls.
  - `LightDetail.svelte`: A dedicated, immersive view for controlling a single light.
  - `Setup.svelte`: The initial authentication flow.
  - `LightCard.svelte` & `ScenePill.svelte`: Reusable UI atoms.
- **Routes**: Located in `src/routes/`.
  - `src/routes/+page.svelte`: Main Dashboard.
  - `src/routes/settings/+page.svelte`: Settings page.
- **State Management**:
  - **Global State**: `src/lib/lightState.svelte.ts` uses Svelte 5 Runes to manage a global cache of lights and scenes, enabling instant navigation and preventing content flash.
  - **Local State**: Individual components use `$state` for UI-specific logic (e.g., form inputs, toggle states).
- **API Client**: `src/lib/lifx.ts` contains the TypeScript definitions and wrapper functions for invoking Tauri commands.

## Styling & Design System

The application uses **Tailwind CSS v4** with a custom theme defined in `src/app.css`. We use semantic CSS variables for consistent theming across the "Glass" aesthetic.

- **Theme Configuration**: Located in the `@theme` block of `src/app.css`.
- **Key Tokens**:
  - `--color-surface`: Main background (`#121212`).
  - `--color-glass`: Standard glass panel background.
  - `--color-text-secondary`: Muted text color.
- **Usage**: Use utility classes like `bg-surface`, `bg-glass`, `text-text-secondary` instead of hardcoded values.

## Backend (Rust)

The backend handles secure communication with the LIFX API and manages application state.

- **Entry Point**: `src-tauri/src/lib.rs` initializes the Tauri application, registers plugins, and exposes commands.
- **LIFX Module**: `src-tauri/src/lifx.rs` contains the core logic for interacting with the LIFX API.
  - `LifxState`: A thread-safe struct holding the HTTP client and API token.
  - `Light`: Struct representing a LIFX light, now including a `connected` boolean for offline status.
  - `Scene`, `Color`: Structs representing other LIFX data models.

## Data Flow

1. **Initialization**: The app starts, and the user provides a LIFX API token.
2. **Command Invocation**: The frontend calls a Tauri command (e.g., `get_lights`) via `invoke`.
3. **Backend Processing**:
    - The command handler in `lib.rs` receives the request.
    - It calls the appropriate method on `LifxState` in `lifx.rs`.
    - `LifxState` uses `reqwest` to make an HTTP request to the LIFX Cloud.
4. **Response**: The LIFX API responds, and the Rust backend deserializes the JSON into Rust structs.
5. **Return to UI**: The Rust structs are serialized back to JSON and returned to the frontend promise.
