# GEMINI.md

## Project Overview

`tauri-applight-effects` is a desktop application built with Tauri v2, SvelteKit, and Rust to control LIFX smart lights. It allows users to view lights, toggle power, change states, and activate scenes.

## Tech Stack

- **Runtime**: Bun v1.3+
- **Frontend**:
  - Svelte v5 (SvelteKit)
  - TypeScript
  - Tailwind CSS v4
  - Vite
- **Backend**:
  - Rust
  - Tauri v2
- **State Management**:
  - Frontend: Svelte Runes (`$state`, `$derived`) + Global Store (`lightState.svelte.ts`)
  - Backend: `LifxState` (Rust), `tauri-plugin-store` (Persistence)

## Development

### Prerequisites

- Rust (stable)
- Bun
- Node.js (optional, Bun is preferred)

### Commands

- **Install Dependencies**: `bun install`
- **Run Development Server**: `bun tauri dev`
- **Build for Production**: `bun tauri build`
- **Check Types**: `bun run check`

## Architecture

The application follows a standard Tauri architecture with a clear separation between Frontend (UI) and Backend (Core Logic).

### Frontend (`src/`)

- **Framework**: SvelteKit adapter-static (SPA mode).
- **Styling**: Tailwind CSS v4 configured in `src/app.css`.
- **Components**: Located in `src/components/`.
- **Routes**: Located in `src/routes/`.
  - `/`: Main Dashboard.
  - `/settings`: Settings page.
- **LIFX Client**: `src/lib/lifx.ts` handles client-side logic and invokes Tauri commands.

### Backend (`src-tauri/`)

- **Entry Point**: `src-tauri/src/lib.rs` exposes Tauri commands.
- **LIFX Logic**: `src-tauri/src/lifx.rs` handles direct communication with LIFX HTTP API.
- **Commands**:
  - `set_api_token`: Stores the LIFX API token.
  - `get_lights`: Fetches list of lights.
  - `set_light_state`: Updates light color/power.
  - `toggle_light`: Toggles light power.
  - `get_scenes`: Fetches available scenes.
  - `activate_scene`: Activates a specific scene.

## Key Files

- `package.json`: Frontend dependencies and scripts.
- `src-tauri/tauri.conf.json`: Tauri configuration (permissions, bundle settings).
- `src-tauri/Cargo.toml`: Rust dependencies.
- `src/lib/lifx.ts`: Frontend interface for Tauri commands.
- `src-tauri/src/lib.rs`: Command registration and state management.

## Conventions

- **Package Manager**: Use `bun` for all script executions.
- **Styling**: Use Tailwind utility classes. Avoid custom CSS unless necessary.
- **Reactivity**: Use Svelte 5 Runes (`$state`) instead of legacy stores where possible.
- **IPC**: All backend communication must go through Tauri commands defined in `lib.rs`.
