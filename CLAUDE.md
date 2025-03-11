# Nyx Development Guide

## Build & Run Commands
- Install prerequisites: `rustup target add wasm32-unknown-unknown && cargo install trunk`
- Run development server: `./run` or `trunk serve --port 3000`
- Build for production: `trunk build --release`
- Format code: `leptosfmt src/*.rs` or `cargo fmt`
- Lint code: `cargo clippy`
- Run tests: `cargo test`

## Code Style Guidelines
- **Formatting**: Use `leptosfmt` for Leptos code, configured in `rust-analyzer.toml`
- **Imports**: Group by standard library, external crates, and internal modules
- **Components**: Use `#[component]` attributes and return `impl IntoView`
- **Naming**: snake_case for functions/methods, PascalCase for types/components
- **State Management**: Use `RwSignal<T>` for reactive state, separate read/write when appropriate
- **Error Handling**: Implement proper error handling with `Result<T, E>` types
- **Database**: Use PostgreSQL with provided scripts in `db/` directory

## Project Structure
- Rust edition 2021, version 1.85
- Leptos framework with client-side rendering
- Uses web_sys for DOM interaction