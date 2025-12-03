use crate::{
    context::Context,
    types::node::{NodeRunner, NodeRunnerFactory},
};

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
    async fn run(&self, _ctx: &Context, _param: serde_json::Value) -> Result<(), String> {
        // nothing need to do
        log::info!("Start workflow!");
        Ok(())
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
    fn create(&self) -> Box<dyn NodeRunner> {
        Box::new(StartRunner::new())
    }
}
