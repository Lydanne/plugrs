use plugrs::interface::Plugin;
use plugrs::macros::export_plugin;

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

impl Default for ExamplePlugin {
    fn default() -> Self {
        Self::new()
    }
}
