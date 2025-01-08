use plugin_interface::Plugin;
use plugin_macros::export_plugin;

#[export_plugin]
pub struct ExamplePlugin;

impl ExamplePlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for ExamplePlugin {
    fn name(&self) -> String {
        "Example Plugin".to_string()
    }

    fn execute(&self) -> i32 {
        println!("执行示例插件");
        42
    }
}
