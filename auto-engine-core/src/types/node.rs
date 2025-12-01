use std::collections::HashMap;
use schemars::Schema;
use serde::{Deserialize, Serialize};
use crate::context::Context;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NodeName {
    pub zh: String,
    pub en: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NodeType {
    pub action_type: String,
    pub name: NodeName,
    pub icon: String,
    pub output_schema: HashMap<String, String>,
    pub input_schema: HashMap<String, String>,
}

pub trait NodeDefine: Send + Sync {
    fn action_type(&self) -> String;

    fn name(&self) -> NodeName;

    fn icon(&self) -> String;

    fn output_schema(&self) -> HashMap<String, String>;

    fn input_schema(&self) -> HashMap<String, String>;
}

#[async_trait::async_trait]
pub trait NodeRunner {
    async fn run(&self, ctx: &Context, param: serde_json::Value) -> Result<(), String>;
}

pub trait NodeRunnerFactory: Send + Sync {
    fn create(&self) -> Box<dyn NodeRunner>;
}
