use schemars::Schema;

use crate::context::Context;

#[derive(Clone)]
pub struct NodeName {
    pub zh: String,
    pub en: String,
}

pub trait NodeDefine: Send + Sync {
    fn action_type(&self) -> String;

    fn name(&self) -> NodeName;

    fn icon(&self) -> String;

    fn output_schema(&self) -> Schema;

    fn input_schema(&self) -> Schema;
}

#[async_trait::async_trait]
pub trait NodeRunner {
    async fn run(&self, ctx: &Context, param: serde_json::Value) -> Result<(), String>;
}

pub trait NodeRunnerFactory: Send + Sync {
    fn create(&self) -> Box<dyn NodeRunner>;
}
