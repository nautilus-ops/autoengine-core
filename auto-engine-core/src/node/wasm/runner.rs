use std::collections::HashMap;
use crate::Plugin;
use crate::context::Context;
use crate::plugin::loader::PluginState;
use crate::types::node::{NodeRunner, NodeRunnerControl, NodeRunnerController, NodeRunnerFactory};
use std::sync::{Arc, Mutex};
use wasmtime::Store;

pub struct Params;

struct PluginRuntime {
    store: Store<PluginState>,
    plugin: Plugin,
}

pub struct WasmRunner {
    runtime: Arc<Mutex<PluginRuntime>>,
}

#[async_trait::async_trait]
impl NodeRunner for WasmRunner {
    type ParamType = serde_json::Value;

    async fn run(&mut self, _ctx: &Context, _param: serde_json::Value) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let mut runtime = self
            .runtime
            .lock()
            .map_err(|err| format!("Failed to lock plugin runtime: {err}"))?;

        let handle_result = {
            let PluginRuntime { plugin, store } = &mut *runtime;
            plugin
                .call_node_handle(store)
                .map_err(|err| format!("Failed to run wasm node: {:?}", err))?
        };

        match handle_result {
            Ok(msg) => log::info!("Wasm node executed: {}", msg),
            Err(err) => return Err(format!("Wasm node execution failed: {}", err)),
        }

        Ok(None)
    }
}

pub struct WasmRunnerFactory {
    runtime: Arc<Mutex<PluginRuntime>>,
}

impl WasmRunnerFactory {
    pub(crate) fn new(store: Store<PluginState>, plugin: Plugin) -> Self {
        Self {
            runtime: Arc::new(Mutex::new(PluginRuntime { store, plugin })),
        }
    }
}

impl NodeRunnerFactory for WasmRunnerFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(WasmRunner {
            runtime: self.runtime.clone(),
        }))
    }
}
