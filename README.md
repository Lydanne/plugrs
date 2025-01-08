# Rust Plugin System

基于 Rust 实现的动态插件系统，支持运行时加载插件。

## 项目结构

```
rust-plugin-system/
├── Cargo.toml                          # 工作空间配置
├── examples/                           # 示例程序
│   └── hello/                         # hello 示例
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── crates/                            # 核心代码
    ├── plugin-interface/              # 插件接口定义
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── plugin-macros/                 # 插件过程宏
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── plugin-host/                   # 插件加载器
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       └── plugin_loader.rs
    └── plugins/                       # 插件集合
        ├── example-plugin/            # 示例插件
        │   ├── Cargo.toml
        │   └── src/
        │       └── lib.rs
        └── file-handler/             # 文件分析插件
            ├── Cargo.toml
            └── src/
                └── lib.rs
```

## 功能特性

- 基于 Rust ABI 的插件系统
- 运行时动态加载插件
- 类型安全的插件接口
- 跨平台支持 (Windows, macOS, Linux)
- 过程宏简化插件开发
- 完整的错误处理

## 快速开始

### 构建项目

```bash
# 克隆项目
git clone <repository-url>
cd rust-plugin-system

# 构建所有包（包括插件）
cargo build

# 运行示例程序
cargo run -p hello
```

## 创建新插件

1. 在 `crates/plugins` 目录下创建新的插件目录：

```bash
mkdir -p crates/plugins/your-plugin/src
```

2. 创建插件的 `Cargo.toml`：

```toml
[package]
name = "your-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
plugin-interface = { path = "../../plugin-interface" }
plugin-macros = { path = "../../plugin-macros" }
```

3. 实现插件 (`src/lib.rs`)：

```rust
use plugin_interface::Plugin;
use plugin_macros::export_plugin;

#[export_plugin]
pub struct YourPlugin;

impl YourPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for YourPlugin {
    fn name(&self) -> String {
        "Your Plugin".to_string()
    }

    fn execute(&self) -> i32 {
        println!("执行你的插件");
        42
    }
}
```

4. 在工作空间的 `Cargo.toml` 中添加插件：

```toml
[workspace]
members = [
    "crates/plugin-interface",
    "crates/plugin-macros",
    "crates/plugin-host",
    "crates/plugins/example-plugin",
    "crates/plugins/file-handler",
    "crates/plugins/your-plugin",  # 添加新插件
    "examples/hello"
]
```

## 插件系统架构

### 核心组件

1. **插件接口** (`plugin-interface`)

   - 定义 `Plugin` trait
   - 提供插件类型定义

2. **插件宏** (`plugin-macros`)

   - 提供 `#[export_plugin]` 属性宏
   - 自动生成插件导出代码

3. **插件加载器** (`plugin-host`)
   - 提供动态库加载功能
   - 管理插件生命周期

### 示例插件

1. **简单示例** (`example-plugin`)

   - 演示基本插件实现
   - 展示插件注册过程

2. **文件分析器** (`file-handler`)
   - 展示实际应用场景
   - 包含状态管理和错误处理

## 开发说明

### 插件接口

插件需要实现 `Plugin` trait：

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> String;
    fn execute(&self) -> i32;
}
```

### 注意事项

- 插件必须使用 `cdylib` crate 类型
- 使用 `#[export_plugin]` 属性宏导出插件
- 插件实现需要是线程安全的 (Send + Sync)
- 动态库会被编译到 `target/debug` 或 `target/release` 目录

## 贡献指南

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
