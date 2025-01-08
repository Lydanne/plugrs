mod plugin_loader;

pub use plugin_loader::PluginLoader;
use std::path::PathBuf;

/// 获取平台特定的动态库文件名
pub fn get_plugin_path(name: &str) -> PathBuf {
    let file_name = match std::env::consts::OS {
        "windows" => format!("{}.dll", name),
        "macos" => format!("lib{}.dylib", name),
        _ => format!("lib{}.so", name),
    };
    // 使用 target/debug 目录
    PathBuf::from("target/debug").join(file_name)
}

/// 插件管理器
pub struct PluginManager {
    plugins: Vec<Box<dyn plugrs_interface::Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// 加载插件
    pub fn load_plugin(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("尝试加载插件: {:?}", path);
        let loader = PluginLoader::new(path)?;
        let plugin = loader.load_plugin()?;
        self.plugins.push(plugin);
        Ok(())
    }

    /// 执行所有已加载的插件
    pub fn execute_all(&self) -> Vec<i32> {
        self.plugins
            .iter()
            .map(|plugin| {
                println!("执行插件: {}", plugin.name());
                plugin.execute()
            })
            .collect()
    }

    /// 获取已加载的插件数量
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
