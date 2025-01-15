# plugrs-host

Plugin host implementation for the plugrs system.

## Features

- Dynamic plugin loading
- Plugin lifecycle management
- Thread-safe plugin execution
- Error handling and recovery

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plugrs-host = "0.1.0"
plugrs-interface = "0.1.0"
```

## Example

```rust
use plugrs_host::PluginManager;
use plugrs_interface::Plugin;

fn main() {
    let manager = PluginManager::new();

    // Load a plugin from a dynamic library or plugin_path(crate)
    let plugin = manager.load_plugin("path/to/plugin.so").unwrap();

    // Execute the plugin
    let result = plugin.execute();
    println!("Plugin result: {}", result);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
