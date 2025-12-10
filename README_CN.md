# AutoEngine Core

AutoEngine Core 是一个以“工作流编排”为中心的异步 Rust 引擎：通过节点图（workflow）描述自动化步骤，统一调度节点能力（图像识别、鼠标/键盘、定时等待、WASM 插件等）与条件控制，并在需要时向上层 UI 发出事件。你可以将其嵌入 GUI，也可以无头运行，用以构建自动化助手或脚本转换工具。

## Highlights（聚焦编排）
- **DAG 编排**：Workflow 由节点列表与连接关系构成，入口节点通常为 `Start`，内置动作节点覆盖图像识别、鼠标移动/点击、键盘输入、定时等待，以及可插拔的 WASM 节点；支持自定义能力扩展。
- **上下文驱动的数据流**：内置 `Context` 支持 `${var:default}` 插值，节点输出自动写入，后续节点零样板复用坐标、状态或自定义字段。
- **运行时控制**：`WorkflowRunner` 提供异步执行、重试/间隔、循环运行、最小时间片约束与取消令牌，适合长时间运行的自动化任务。
- **可观察性事件**：可选 `tauri` 特性输出节点级事件（running/skip/done/error/cancel），方便 UI 可视化进度与结果。
- **能力插件化**：图像匹配基于 OpenCV，输入输出动作通过 Enigo，但都作为“节点能力”注入，编排层保持简单和可替换。

## How It Works
1. **Workflow 定义**：workflow 是有向无环图（DAG），`nodes` 描述动作与元数据（名称、重试、间隔、条件等），`connections` 负责连接 `from` 与 `to`。资源文件建议存放在 workflow 旁的 `files/` 目录（如 `workflow/files/*.png`）。
2. **上下文与变量**：节点将检测结果或计算值写入 Context，后续以 `${detect-dot.dot.png.x}` 方式读取，支持默认值 `${value:0}`。
3. **条件与分支**：节点支持存在/不存在判定与表达式（`condition: "${foo} > 10"`），无须额外脚本即可控制分支与短路。
4. **执行器**：`WorkflowRunner` 负责异步调度、重试、节流、取消与循环执行；当需要 UI 反馈时，可开启 tauri 事件输出。

## Getting Started
前置条件：
- Rust 1.80+（工作区使用 2024 edition）。
- OpenCV 运行时（[`opencv` crate](https://docs.rs/opencv) 需要系统级库，可通过 `brew install opencv` 或 `apt install libopencv-dev` 安装）。
- （可选）Tauri 2.x：需要事件上报或主线程键盘处理时开启。

构建与测试：

```bash
cargo build -p auto-engine-core
cargo test -p auto-engine-core
```

### Feature Flags
Workflow 编排为核心能力；下列 feature flags 用于裁剪能力（默认全部启用）：

| Feature    | Description |
|------------|-------------|
| `types`    | Workflow/node 数据结构。 |
| `context`  | 共享上下文及变量解析。 |
| `event`    | 节点事件定义。 |
| `runner`   | 节点能力执行器（鼠标、键盘、图像识别等）。 |
| `utils`    | 变量解析等辅助工具。 |
| `tauri`    | 主线程键盘处理与事件上报。 |

可在 `Cargo.toml` 中按需关闭，例如设置 `default-features = false` 以获得最小构建。

## 贡献指南
1. 在提交前确保 `cargo fmt` 与 `cargo clippy` 通过。
2. 新增节点、条件或工具函数时记得补充测试（`cargo test`）。
3. 若扩展了 workflow 动作或 DSL，请同步更新本文档。

## 维护者
项目由 CeerDecy（袁浩楠）维护，邮箱：ceerdecy@gmail.com。

## 许可证
项目基于 [Apache License 2.0](LICENSE) 发布。
