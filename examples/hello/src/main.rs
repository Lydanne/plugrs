use plugin_host::{get_plugin_path, PluginManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建插件管理器
    let mut manager = PluginManager::new();

    // 加载示例插件
    manager.load_plugin(get_plugin_path("example_plugin"))?;
    println!("加载了 {} 个插件", manager.plugin_count());

    // 加载文件分析插件
    manager.load_plugin(get_plugin_path("file_handler"))?;
    println!("加载了 {} 个插件", manager.plugin_count());

    // 执行所有插件
    let results = manager.execute_all();

    // 显示执行结果
    println!("\n插件执行结果:");
    println!("----------------------------------------");
    for (i, result) in results.iter().enumerate() {
        println!("插件 {}: 返回值 = {}", i + 1, result);
    }
    println!("----------------------------------------");

    Ok(())
}
