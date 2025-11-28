use crate::Plugin;
use crate::node::wasm::node::WasmNode;
use crate::node::wasm::runner::WasmRunnerFactory;
use crate::node_register::host;
use crate::node_register::host::Node;
use crate::register::bus::NodeRegisterBus;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use wasmtime::Engine;
use wasmtime::Store;
use wasmtime::component::{Component, HasSelf, Linker, ResourceTable};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

pub(crate) struct PluginState {
    pub(crate) node_bus: Arc<RwLock<NodeRegisterBus>>,
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for PluginState {
    fn ctx(&mut self) -> WasiCtxView {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

impl host::Host for PluginState {
    fn invent_entirely_new_node(&mut self, n: Node) -> () {
        let mut bus = self.node_bus.write().unwrap();
        let name = n.action_type.clone();
        bus.register_node(name.clone(), Box::new(WasmNode::from_node(n)));
        log::info!("Node: {} register success", name)
    }
}

pub struct PluginLoader {}

impl PluginLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_plugins(
        &mut self,
        plugin_folder: PathBuf,
        bus: Arc<RwLock<NodeRegisterBus>>,
    ) -> anyhow::Result<()> {
        let engine = wasmtime::Engine::default();

        if !plugin_folder.exists() {
            std::fs::create_dir_all(&plugin_folder)?;
        }

        for entry in std::fs::read_dir(plugin_folder)? {
            let path = entry?.path();
            if path.is_file() && path.extension().and_then(OsStr::to_str) == Some("wasm") {
                self.load_plugin(&engine, path, bus.clone())?;
            }
        }

        Ok(())
    }
    fn load_plugin(
        &mut self,
        engine: &Engine,
        plugin_path: PathBuf,
        bus: Arc<RwLock<NodeRegisterBus>>,
    ) -> anyhow::Result<()> {
        let component = Component::from_file(engine, &plugin_path)?;
        let linker = Self::build_linker(engine)?;
        let mut store = Self::build_store(engine, bus.clone())?;

        let plugin = Plugin::instantiate(&mut store, &component, &linker)?;
        plugin.call_init(&mut store).map_err(|e| {
            log::error!("Failed to call plugin init: {:?}", e);
            e
        })?;

        let plugin_name = plugin.call_get_plugin_name(&mut store)?;
        let factory = WasmRunnerFactory::new(store, plugin);

        {
            let mut bus = bus.write().unwrap();
            bus.register_runner(plugin_name, Box::new(factory));
        }

        Ok(())
    }

    fn build_store(
        engine: &Engine,
        node_bus: Arc<RwLock<NodeRegisterBus>>,
    ) -> anyhow::Result<Store<PluginState>> {
        let wasi = WasiCtx::builder().inherit_stdio().inherit_args().build();
        Ok(Store::new(
            engine,
            PluginState {
                node_bus: node_bus.clone(),
                wasi_ctx: wasi,
                resource_table: ResourceTable::new(),
            },
        ))
    }

    fn build_linker(engine: &Engine) -> anyhow::Result<Linker<PluginState>> {
        let mut linker = Linker::new(engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        host::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
        Ok(linker)
    }
}
