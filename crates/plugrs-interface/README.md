# plugrs-interface

Plugin interface definitions for the plugrs system.

## Features

- Type-safe plugin interface
- Thread-safe plugin trait
- Zero-cost abstractions

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plugrs-interface = "0.1.0"
```

## Example

```rust
use plugrs_interface::Plugin;

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
