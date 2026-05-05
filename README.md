# envlayer

Composable environment variable management with layered overrides for local and CI contexts.

---

## Installation

Add `envlayer` to your `Cargo.toml`:

```toml
[dependencies]
envlayer = "0.1"
```

---

## Usage

`envlayer` lets you define multiple environment layers that are merged in priority order — later layers override earlier ones.

```rust
use envlayer::{EnvLayer, LayerStack};

fn main() {
    let stack = LayerStack::new()
        .push(EnvLayer::from_file(".env"))          // base defaults
        .push(EnvLayer::from_file(".env.local"))    // local overrides
        .push(EnvLayer::from_env());                // CI / process env wins

    let config = stack.resolve();

    println!("DATABASE_URL = {}", config.get("DATABASE_URL").unwrap());
    println!("LOG_LEVEL    = {}", config.get("LOG_LEVEL").unwrap_or("info"));
}
```

**Layer priority** (highest to lowest):

1. Process environment variables
2. `.env.local`
3. `.env`

Missing files are silently skipped, making the same binary work seamlessly on a developer laptop and in CI without any code changes.

---

## Features

- 🗂 **Layered merging** — stack as many sources as you need
- 🔇 **Graceful missing files** — no panics on absent `.env` files
- 🔒 **Read-only resolved map** — prevents accidental mutation after resolution
- 🚀 **Zero unsafe code**

---

## License

This project is licensed under the [MIT License](LICENSE).