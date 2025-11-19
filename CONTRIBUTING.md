# Contributing to Light Effects

Thank you for your interest in contributing to Light Effects! We welcome contributions from the community.

## Development Setup

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/).
- **Node.js / Bun**: This project uses [Bun](https://bun.sh/) as the package manager.
- **Tauri CLI**: Install via `cargo install tauri-cli` or use the bundled CLI.

### Installation

1. **Clone the repository**:

    ```bash
    git clone https://github.com/yourusername/light-effects.git
    cd light-effects
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

- **Frontend**: We use Prettier for code formatting. Run `bun format` to format your code.
- **Styling**: Use the semantic theme classes defined in `src/app.css` (e.g., `bg-glass`, `text-text-secondary`) instead of hardcoded colors.
- **Backend**: We use `rustfmt`. Run `cargo fmt` in the `src-tauri` directory.

## Submitting Changes

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Make your changes and commit them.
4. Push to your fork and submit a Pull Request.
