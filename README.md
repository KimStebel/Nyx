# Nyx

A todo list app using foldable nested lists and contenteditable for everything

## Prerequisites

* rustup
* cargo install trunk --version 0.21.9
* cargo install wasm-pack --version 0.13.1
* rustup target add wasm32-unknown-unknown

## Run

```
./run
```

## Testing

The project includes unit tests for components and models. Tests are written using the `wasm-bindgen-test` framework which allows testing WebAssembly code in a browser environment.

### Running Tests

```bash
wasm-pack test --headless --chrome
```

You can also run with Firefox or Safari by replacing `--chrome` with `--firefox` or `--safari`.

