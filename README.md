# Oculus for LIFX

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Svelte](https://img.shields.io/badge/svelte-v5-orange.svg)
![Tauri](https://img.shields.io/badge/tauri-v2-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-black.svg)
![Bun](https://img.shields.io/badge/bun-v1.3-white.svg)

Oculus is a desktop controller for LIFX lights, built with Tauri, Svelte, and Rust. Inspired by the World Trade Center Oculus, it provides a sleek, modern interface for managing your smart lights and activating scenes.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Tech Stack](#tech-stack)
- [Building for Production](#building-for-production)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Features

- 🎨 **Glassmorphic UI**: A stunning, dark-mode interface with dynamic lighting effects and frosted glass elements.
- 🎛️ **Interactive Dashboard**:
  - **Global Control**: Master switch to toggle all lights instantly.
  - **Live Status**: "Mini Orbs" on each card show real-time color and brightness.
  - **Offline Detection**: Visual indicators for offline lights with disabled controls.
  - **Instant Navigation**: Smart caching for zero-latency page transitions.
  - **Rich Scenes**: Activate your favorite scenes with one click.
- 💡 **Deep Light Control**:
  - **Dynamic Visualization**: A central orb pulses with the light's actual color.
  - **Precision Tools**: Fine-tune Hue, Saturation, Brightness, and Kelvin with custom sliders.
- 🔒 **Secure Setup & Settings**:
  - Easy, secure token entry to connect your LIFX account.
  - Dedicated settings page to manage your API token and view app info.
- 🖥️ **Cross-Platform**: Runs on macOS, Windows, and Linux (via Tauri).

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

## Usage

1. **Launch the App**: Run `bun tauri dev` or open the built application.
2. **Setup API Token**: On the first launch, you will be prompted to enter your LIFX API token. You can generate one in your [LIFX Cloud Account](https://cloud.lifx.com/settings).
3. **Control Lights**: Use the dashboard to toggle lights or click on a card to access detailed controls.
4. **Activate Scenes**: Click on any scene in the "Scenes" section to activate it.

## Tech Stack

- **Frontend**: [Svelte 5](https://svelte.dev/), [Tailwind CSS 4](https://tailwindcss.com/)
- **Backend**: [Rust](https://www.rust-lang.org/), [Tauri 2](https://tauri.app/)
- **Runtime**: [Bun 1.3](https://bun.sh/)

## Building for Production

To build the application for your platform:

```bash
bun tauri build
```

The build artifacts will be located in `src-tauri/target/release/bundle/`.

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Inspired by the architectural design of the World Trade Center Oculus.
- Built with the amazing [Tauri](https://tauri.app/) framework.
