use crate::types::node::{NodeRunnerControl, NodeRunnerController};
use crate::{
    context::Context,
    types::node::{NodeRunner, NodeRunnerFactory},
};
use std::collections::HashMap;

pub struct Params;

#[derive(Default)]
pub struct StartRunner;

impl StartRunner {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl NodeRunner for StartRunner {
    type ParamType = serde_json::Value;

    async fn run(
        &mut self,
        _ctx: &Context,
        _param: serde_json::Value,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        // nothing need to do
        log::info!("Start workflow!");
        Ok(None)
    }
}

#[derive(Default)]
pub struct StartRunnerFactory;

impl StartRunnerFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeRunnerFactory for StartRunnerFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(StartRunner::new()))
    }
}
