# AutoEngine Core

AutoEngine Core is an asynchronous Rust engine centered on workflow orchestration: it describes automation steps with workflow graphs, schedules node capabilities (image recognition, mouse/keyboard, timed waits, WASM plugins, etc.) and condition controls, and emits events to upper-layer UIs when needed. You can embed it in a GUI or run it headless to build automation assistants or script conversion tools.

## Highlights (Orchestration Focus)
- **DAG orchestration**: A workflow consists of a node list and connections. The entry node is usually `Start`; built-in action nodes cover image recognition, mouse move/click, keyboard input, timed waits, and pluggable WASM nodes. Custom capabilities are supported.
- **Context-driven data flow**: The built-in `Context` supports `${var:default}` interpolation. Node outputs are automatically written so later nodes can reuse coordinates, state, or custom fields with zero boilerplate.
- **Runtime controls**: `WorkflowRunner` offers async execution, retries/intervals, looping, minimum timeslice constraints, and cancellation tokens for long-running automation tasks.
- **Observable events**: The optional `tauri` feature emits node-level events (running/skip/done/error/cancel) so UIs can visualize progress and results.
- **Pluggable capabilities**: Image matching is backed by OpenCV and I/O actions use Enigo, but both are injected as “node capabilities” so the orchestration layer stays simple and replaceable.

## How It Works
1. **Workflow definition**: The workflow is a DAG. `nodes` describe actions and metadata (name, retries, intervals, conditions, etc.), and `connections` link `from` to `to`. Resource files are best stored next to the workflow in `files/` (for example `workflow/files/*.png`).
2. **Context and variables**: Nodes write detection results or computed values into `Context`, later read as `${detect-dot.dot.png.x}`. Defaults are supported via `${value:0}`.
3. **Conditions and branching**: Nodes support existence/non-existence checks and expressions (`condition: "${foo} > 10"`), enabling branching and short-circuiting without extra scripts.
4. **Runner**: `WorkflowRunner` handles async scheduling, retries, throttling, cancellation, and looped execution; enable tauri event output when UI feedback is required.

## Getting Started
Prerequisites:
- Rust 1.80+ (edition 2024 workspace).
- OpenCV runtime (the [`opencv` crate](https://docs.rs/opencv) needs system libraries such as `brew install opencv` or `apt install libopencv-dev`).
- Optional: Tauri 2.x when you need event emission or main-thread keyboard handling.

Build and test the workspace:

```bash
cargo build -p auto-engine-core
cargo test -p auto-engine-core
```

### Feature Flags
Workflow orchestration is core; the following feature flags trim capabilities (all enabled by default):

| Feature    | Description |
|------------|-------------|
| `types`    | Workflow/node data structures. |
| `context`  | Shared context and variable resolution. |
| `event`    | Node event definitions. |
| `runner`   | Node capability executors (mouse, keyboard, image recognition, etc.). |
| `utils`    | Helper utilities such as variable parsing. |
| `tauri`    | Main-thread keyboard handling and event emission. |

Disable features in `Cargo.toml` as needed, e.g. set `default-features = false` for a minimal build.

## Contributing
1. Ensure `cargo fmt` and `cargo clippy` pass locally.
2. Add tests (`cargo test`) whenever you introduce new node types, conditions, or utilities.
3. Document new workflow actions or DSL additions in this README.

## Maintainer
This project is maintained by CeerDecy (Yuan Haonan), email: ceerdecy@gmail.com.

## License
This project is licensed under the [Apache License 2.0](LICENSE).
