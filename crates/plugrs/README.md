# plugrs

A lightweight and type-safe plugin system for Rust.

## Features

- Type-safe plugin interface
- Dynamic plugin loading
- Thread-safe plugin execution
- Procedural macros for plugin development
- Error handling and recovery
- Zero-cost abstractions

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plugrs = "0.1.0"
```

## Example

Creating a plugin:

```rust
use plugrs::prelude::*;

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

Loading and using plugins:

```rust
use plugrs::PluginManager;

fn main() {
    let manager = PluginManager::new();

    // Load a plugin from a dynamic library
    let plugin = manager.load_plugin("path/to/plugin.so").unwrap();

    // Execute the plugin
    let result = plugin.execute();
    println!("Plugin result: {}", result);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
