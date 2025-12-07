use crate::context::Context;
use crate::types::node::{NodeRunner, NodeRunnerControl, NodeRunnerController, NodeRunnerFactory};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeWaitParam {
    pub duration: u64,
}

#[derive(Default)]
pub struct TimeWaitRunner;

impl TimeWaitRunner {
    pub fn new() -> Self {
        TimeWaitRunner
    }
}

#[async_trait::async_trait]
impl NodeRunner for TimeWaitRunner {
    type ParamType = TimeWaitParam;

    async fn run(
        &mut self,
        _ctx: &Context,
        param: Self::ParamType,
    ) -> Result<Option<HashMap<String, Value>>, String> {
        log::info!("Running time wait {:?}", param);
        tokio::time::sleep(tokio::time::Duration::from_secs(param.duration)).await;
        Ok(None)
    }
}

#[derive(Default)]
pub struct TimeWaitRunnerFactory;

impl TimeWaitRunnerFactory {
    pub fn new() -> Self {
        TimeWaitRunnerFactory
    }
}

impl NodeRunnerFactory for TimeWaitRunnerFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(TimeWaitRunner::new()))
    }
}
