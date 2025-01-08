/// 插件接口 trait
pub trait Plugin: Send + Sync {
    fn name(&self) -> String;
    fn execute(&self) -> i32;
}

/// 插件创建函数类型
pub type CreatePluginFn = fn() -> Box<dyn Plugin>;
