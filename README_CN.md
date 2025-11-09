# AutoEngine Core

AutoEngine Core 是一个基于异步 Rust 的桌面自动化引擎，用于执行由 YAML 描述的工作流。它能够把图像识别、鼠标/键盘控制、条件逻辑以及可选的 Tauri 事件推送组合在一起，既可以嵌入到 GUI 应用中，也能在无界面环境下运行，为自定义自动化工具提供了灵活的内核。

## 核心特性
- 以 Stage 为单位的管线模型，内置 `Start`、`ImageRecognition`、`MouseMove`、`MouseClick`、`KeyBoard`、`TimeWait` 等节点类型。
- 自带上下文存储和 `${var:default}` 插值语法，方便在后续节点中复用上一阶段生成的数据。
- 基于 OpenCV 的模板匹配，支持自定义缩放与 imread 选项，并通过 `screenshots` crate 实时截屏。
- 依托 Enigo 实现鼠标和键盘操作，覆盖点击、按下/松开、文本输入，同时可配置重试与间隔。
- 可选的 Tauri 集成，在每个节点开始/跳过/完成/出错/取消时发送事件，便于 UI 展示执行状态。
- 基于 Tokio 的执行器，内置取消令牌、速率控制与循环执行能力，适合长期运行的自动化脚本。

## 工作原理
1. **定义管线**：管线由若干 Stage 组成，每个 Stage 含有一个或多个节点（动作）。通常会把模板图片等资源放在 `pipeline/image/*.png` 等目录下。
2. **上下文与变量**：节点可向共享 `Context` 写入数据，后续节点通过 `${stage.image.png.x}` 或 `${value:0}`（带默认值）读取。
3. **条件判断**：节点可指定 `exist`、`not_exist` 或布尔表达式（如 `condition: "${foo} > 10"`）来决定是否执行。
4. **运行器**：`PipelineRunner` 使用异步任务驱动执行，处理重试、最小循环间隔，并通过 `tokio_util::sync::CancellationToken` 接收取消信号。
5. **事件通知**：启用 `tauri` 功能后，会向宿主应用发送结构化节点事件，便于可视化进度。

## 快速开始
前置条件：
- Rust 1.80+（工作区使用 2024 edition）。
- 可用的 OpenCV 编译环境（`opencv` crate 依赖本地库，可通过 `brew install opencv`、`apt install libopencv-dev` 等方式安装）。
- （可选）Tauri 2.x，如果需要将本 crate 嵌入 Tauri 应用。

构建与测试：

```bash
cargo build -p auto-engine-core
cargo test -p auto-engine-core
```

### 示例管线
```yaml
- stage:
    - action_type: Start
      name: "main"
      conditions: ""
- stage:
    - action_type: ImageRecognition
      name: "find-dot"
      retry: -1
      interval: 0
      params:
        images:
          - "dot.png"
        sub_pipeline: ""
- stage:
    - action_type: MouseMove
      name: "Move cursor to dot"
      retry: 2
      params:
        x: "${find-dot.dot.png.x}"
        y: "${find-dot.dot.png.y}"
      conditions:
        exist: "find-dot.dot.png"
```

要点说明：
- 图像识别节点可以同时匹配多张模板，坐标与得分会自动写入上下文。
- 鼠标 / 键盘节点可通过 `${...}` 语法引用这些坐标，并结合条件控制执行。
- `retry` 与 `interval` 允许为每个动作配置容错策略，避免单点失败导致整个管线终止。

### 功能开关
该 crate 暴露多个可选模块，默认全部启用：

| Feature   | 说明 |
|-----------|------|
| `types`   | 节点、Stage、Pipeline 等数据结构。 |
| `context` | 共享上下文及相关工具。 |
| `event`   | 节点事件负载定义。 |
| `pipeline`| 管线调度与执行逻辑。 |
| `runner`  | 鼠标、键盘、图像识别执行器。 |
| `utils`   | 变量解析等辅助工具。 |
| `tauri`   | Tauri 集成（主线程键盘调用、事件推送等）。 |

可在 `Cargo.toml` 中按需关闭，例如设置 `default-features = false` 以获得最小构建。

## 贡献指南
1. 在提交前确保 `cargo fmt` 与 `cargo clippy` 通过。
2. 新增节点、条件或工具函数时记得补充测试（`cargo test`）。
3. 若扩展了管线 DSL，请同步更新本文档。

## 许可证
项目基于 [Apache License 2.0](LICENSE) 发布。
