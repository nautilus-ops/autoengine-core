# AutoEngine Core

AutoEngine Core is an asynchronous Rust engine for driving desktop automation pipelines. It executes YAML workflows that combine image recognition, mouse/keyboard control, conditional logic, and optional Tauri event emission. The crate can be embedded in a GUI application or run headlessly, making it a flexible foundation for custom automation tools.

## Highlights
- Stage-based pipelines expressed in YAML with `Start`, `ImageRecognition`, `MouseMove`, `MouseClick`, `KeyBoard`, and `TimeWait` nodes.
- Built-in context store with `${var:default}` interpolation so later actions can reuse data captured by previous stages.
- OpenCV-powered template matching with configurable resize/imread options plus on-demand screen captures via the `screenshots` crate.
- Mouse and keyboard automation through Enigo, including press/hold, clicks, and text input with retry/backoff controls.
- Optional Tauri integration that emits per-node events (running, skip, done, error, cancel) for UI feedback.
- Tokio-based executor with cancellation tokens, rate limiting, and looped execution for long-running bots.

## How It Works
1. **Pipeline definition** – Pipelines are lists of stages; each stage contains one or more nodes (actions). Pipelines normally live next to their supporting assets (for example, under `pipeline/image/*.png` for template matching).
2. **Context & variables** – Each node can write to the shared `Context`, and later nodes reference those values using `${stage.image.png.x}` style placeholders or default fallbacks (`${value:0}`).
3. **Conditions** – Nodes may specify `exist`, `not_exist`, or boolean expressions (`condition: "${foo} > 10"`) that gate execution.
4. **Runner** – `PipelineRunner` spins up asynchronous tasks, handles retries, enforces minimum loop intervals, and supports cancellation via `tokio_util::sync::CancellationToken`.
5. **Events** – When compiled with the `tauri` feature the engine emits structured node events so host applications can visualize progress.

## Getting Started
Prerequisites:
- Rust 1.80+ (edition 2024 workspace).
- A working OpenCV toolchain (the [`opencv` crate](https://docs.rs/opencv) expects the native libraries to be installed, e.g., `brew install opencv` on macOS or `apt install libopencv-dev` on Debian/Ubuntu).
- (Optional) Tauri 2.x if you plan to embed the crate in a Tauri application.

Build and test the workspace:

```bash
cargo build -p auto-engine-core
cargo test -p auto-engine-core
```

### Example Pipeline
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

Key ideas demonstrated above:
- Image recognition nodes can look for multiple templates; their results (coordinates, scores) are stored automatically in the context.
- Mouse/keyboard nodes can reference those coordinates through `${...}` expressions and gate execution using conditions.
- `retry` and `interval` allow each action to handle flaky inputs without bringing down the entire pipeline.

### Feature Flags
The crate exposes several opt-in modules. By default all of them are enabled:

| Feature    | Description |
|------------|-------------|
| `types`    | Data structures for nodes, stages, and pipelines. |
| `context`  | Shared context store and helpers. |
| `event`    | Node event payloads. |
| `pipeline` | Pipeline runner orchestration. |
| `runner`   | Mouse, keyboard, and image recognition executors. |
| `utils`    | Variable parsing utilities and helpers. |
| `tauri`    | Enables Tauri-specific integrations such as main-thread keyboard handling and event emission. |

Disable features in `Cargo.toml` as needed, e.g. `default-features = false` for a minimal build.

## Contributing
1. Ensure `cargo fmt` and `cargo clippy` pass locally.
2. Add tests (`cargo test`) whenever you introduce new node types, conditions, or utilities.
3. Document new pipeline actions or DSL additions in this README.

## License
This project is licensed under the [Apache License 2.0](LICENSE).
