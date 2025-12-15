use crate::types::node::{NodeRunnerControl, NodeRunnerController, SchemaField};
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
    type ParamType = HashMap<String, serde_json::Value>;

    async fn run(
        &mut self,
        _ctx: &Context,
        param: Self::ParamType,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let val = param.get("params").unwrap_or_default().clone();
        let params: HashMap<String, serde_json::Value> =
            serde_json::from_value(val).unwrap_or_default();

        let mut outputs = HashMap::new();
        for (k, v) in params.iter() {
            outputs.insert(k.clone(), v.clone());
        }

        // outputs
        Ok(Some(outputs))
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
