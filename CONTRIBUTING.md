# 贡献指南

感谢你对 plugrs 项目感兴趣！这份文档将帮助你了解如何参与项目开发。

## 开发环境设置

1. **克隆项目**

```bash
git clone https://github.com/your-username/plugrs.git
cd plugrs
```

2. **运行环境初始化脚本**

```bash
./scripts/setup.sh
```

这个脚本会自动安装和配置所有必要的工具。

## 开发流程

1. **创建分支**

   - 功能分支：`feature/your-feature-name`
   - 修复分支：`fix/issue-description`
   - 文档分支：`docs/what-you-are-documenting`

2. **提交规范**
   我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

- `feat`: 新功能
- `fix`: 修复问题
- `docs`: 文档更改
- `style`: 代码格式修改
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

示例：

```bash
git commit -m "feat: 添加新的插件加载功能"
git commit -m "fix: 修复插件初始化问题"
```

3. **代码规范**

- 所有代码必须通过 `cargo fmt` 格式化
- 必须通过 `cargo clippy` 检查
- 新功能必须包含测试
- 保持代码文档的完整性

4. **测试**

```bash
# 运行所有测试
cargo test --all-features

# 运行特定测试
cargo test test_name
```

## Pull Request 流程

1. **准备工作**

   - 确保你的代码已经完成自测
   - 确保所有测试通过
   - 确保代码符合项目规范

2. **创建 Pull Request**

   - 标题遵循 Conventional Commits 规范
   - 填写完整的描述，说明改动的目的和影响
   - 关联相关的 Issue（如果有）

3. **PR 检查清单**
   - [ ] 代码格式化完成
   - [ ] Clippy 检查通过
   - [ ] 测试用例已添加/更新
   - [ ] 文档已更新
   - [ ] Commit 信息规范

## 插件开发指南

1. **创建新插件**

```rust
use plugrs_interface::Plugin;
use plugrs_macros::export_plugin;

#[export_plugin]
pub struct MyPlugin;

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

2. **插件规范**
   - 实现 `Plugin` trait
   - 使用 `#[export_plugin]` 宏
   - 提供清晰的文档和示例
   - 处理所有可能的错误情况

## 发布流程

1. **版本号规则**
   我们使用语义化版本：

- 主版本号：不兼容的 API 修改
- 次版本号：向下兼容的功能性新增
- 修订号：向下兼容的问题修正

2. **发布检查**

```bash
# 检查发布内容
cargo smart-release --dry-run

# 正式发布
cargo smart-release
```

## 获取帮助

- 提交 Issue 请使用相应的模板
- 有问题可以在 Discussions 中讨论
- 重大变更请先在 Issue 中讨论

## 行为准则

请参阅 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)。

## 许可证

通过提交 PR，你同意你的贡献将按照项目的许可证进行授权。
