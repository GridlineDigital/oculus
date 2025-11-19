# Oculus

Oculus is a desktop controller for LIFX lights, built with Tauri, Svelte, and Rust. Inspired by the World Trade Center Oculus, it provides a sleek, modern interface for managing your smart lights and activating scenes.

## Features

- **Glassmorphic UI**: A stunning, dark-mode interface with dynamic lighting effects and frosted glass elements.
- **Interactive Dashboard**:
  - **Global Control**: Master switch to toggle all lights instantly.
  - **Live Status**: "Mini Orbs" on each card show real-time color and brightness.
  - **Offline Detection**: Visual indicators for offline lights with disabled controls.
  - **Instant Navigation**: Smart caching for zero-latency page transitions.
  - **Rich Scenes**: Activate your favorite scenes with one click.
- **Deep Light Control**:
  - **Dynamic Visualization**: A central orb pulses with the light's actual color.
  - **Precision Tools**: Fine-tune Hue, Saturation, Brightness, and Kelvin with custom sliders.
- **Secure Setup & Settings**:
  - Easy, secure token entry to connect your LIFX account.
  - Dedicated settings page to manage your API token and view app info.
- **Cross-Platform**: Runs on macOS, Windows, and Linux (via Tauri).

## Documentation

- [Architecture Overview](docs/ARCHITECTURE.md)
- [API Documentation](docs/API.md)
- [Contributing Guidelines](CONTRIBUTING.md)

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) or [Bun](https://bun.sh/) (Bun is recommended)

## Getting Started

1. **Clone the repository**:

    ```bash
    git clone https://github.com/yourusername/light-effects.git
    cd light-effects
    ```

2. **Install dependencies**:

    ```bash
    bun install
    ```

3. **Run the application**:

    ```bash
    bun tauri dev
    ```

## Building for Production

To build the application for your platform:

```bash
bun tauri build
```

The build artifacts will be located in `src-tauri/target/release/bundle/`.

## License

MIT
