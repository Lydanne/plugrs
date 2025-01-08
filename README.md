# Plugrs

一个轻量级、类型安全的 Rust 插件系统。

## 项目结构

```
plugrs/
├── Cargo.toml                 # 工作空间配置
├── crates/                    # 核心代码
│   ├── plugrs-interface/     # 插件接口定义
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── plugrs-macros/       # 插件过程宏
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── plugrs-host/         # 插件加载器
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── main.rs
│   │       └── plugin_loader.rs
│   └── plugrs/              # 主库
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── plugins/                  # 示例插件
│   ├── example-plugin/      # 基础示例插件
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── file-handler/        # 文件处理插件
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── examples/                # 示例程序
    └── hello/              # 基础示例
        ├── Cargo.toml
        └── src/
            └── main.rs
```

## 特性

- 🚀 简单易用的 API
- 🔒 类型安全的插件接口
- 🔌 运行时动态加载
- 🛠 过程宏简化插件开发
- 📦 跨平台支持
- ⚡ 零成本抽象

## 快速开始

### 安装

```toml
[dependencies]
plugrs = "0.1.0"
```

### 使用示例

1. **创建插件**

```rust
use plugrs::prelude::*;

#[export_plugin]
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> String {
        "My Plugin".to_string()
    }

    fn execute(&self) -> i32 {
        println!("Hello from plugin!");
        42
    }
}
```

2. **加载和使用插件**

```rust
use plugrs::{get_plugin_path, PluginManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建插件管理器
    let mut manager = PluginManager::new();

    // 加载插件
    manager.load_plugin(get_plugin_path("my_plugin"))?;
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
```

## 插件开发指南

### 1. 创建新插件

```bash
cargo new --lib my-plugin
cd my-plugin
```

### 2. 配置 Cargo.toml

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
plugrs = "0.1.0"
```

### 3. 实现插件

```rust
use plugrs::prelude::*;

#[export_plugin]
pub struct MyPlugin;

impl Default for MyPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for MyPlugin {
    fn name(&self) -> String {
        "My Plugin".to_string()
    }

    fn execute(&self) -> i32 {
        // 实现你的插件逻辑
        42
    }
}
```

### 4. 构建和测试

```bash
# 构建插件
cargo build --release

# 运行示例程序
cargo run --example hello
```

## API 文档

详细的 API 文档请访问 [docs.rs/plugrs](https://docs.rs/plugrs)。

## 示例

查看 [examples](./examples) 目录获取更多示例：

- `hello`: 基础示例，展示如何加载和执行插件
- `example-plugin`: 简单的示例插件实现
- `file-handler`: 更复杂的文件处理插件示例

## 贡献指南

欢迎贡献！请查看 [CONTRIBUTING.md](./CONTRIBUTING.md) 了解如何参与项目开发。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](./LICENSE) 文件。
