# Contributing to Oculus

Thank you for your interest in contributing to Oculus! We welcome contributions from the community.

## Development Setup

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/).
- **Node.js / Bun**: This project uses [Bun](https://bun.sh/) as the package manager.
- **Tauri CLI**: The project uses the local Tauri CLI via `bun tauri`.

### Installation

1. **Clone the repository**:

    ```bash
    git clone https://github.com/GridlineDigital/oculus.git
    cd oculus
    ```

2. **Install dependencies**:

    ```bash
    bun install
    ```

3. **Run the development server**:

    ```bash
    bun tauri dev
    ```

    This command will start the frontend dev server and the Tauri application window.

## Project Structure

- `src/`: SvelteKit frontend code.
- `src-tauri/`: Rust backend code.
- `docs/`: Project documentation.

## Code Style

- **Type Checking**: Run `bun check` to verify TypeScript types.
- **Styling**: Use the semantic theme classes defined in `src/app.css` (e.g., `bg-glass`, `text-text-secondary`) instead of hardcoded colors.
- **Backend**: We use `rustfmt`. Run `cargo fmt` in the `src-tauri` directory.

## Submitting Changes

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Make your changes and commit them.
4. Push to your fork and submit a Pull Request.
