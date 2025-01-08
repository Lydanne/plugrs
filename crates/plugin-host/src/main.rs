use crate::plugin_loader::PluginLoader;
use std::path::PathBuf;

mod plugin_loader;

fn get_plugin_path() -> PathBuf {
    let file_name = match std::env::consts::OS {
        "windows" => "example_plugin.dll",
        "macos" => "libexample_plugin.dylib",
        _ => "libexample_plugin.so",
    };
    PathBuf::from(file_name)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = PluginLoader::new(get_plugin_path())?;
    let plugin = loader.load_plugin()?;

    println!("加载插件: {}", plugin.name());
    let result = plugin.execute();
    println!("插件执行结果: {}", result);

    Ok(())
}
