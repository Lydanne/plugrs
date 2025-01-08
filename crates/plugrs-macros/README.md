# plugrs-macros

Procedural macros for the plugrs system.

## Features

- `#[plugin]` macro for implementing plugin traits
- Automatic plugin registration
- Type-safe plugin interface generation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plugrs-macros = "0.1.0"
plugrs-interface = "0.1.0"
```

## Example

```rust
use plugrs_interface::Plugin;
use plugrs_macros::plugin;

#[plugin]
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> String {
        "My Plugin".to_string()
    }

    fn execute(&self) -> i32 {
        42
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
