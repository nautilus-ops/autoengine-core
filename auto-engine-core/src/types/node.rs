use crate::context::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct I18nValue {
    pub zh: String,
    pub en: String,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct NodeType {
    pub action_type: String,
    pub name: I18nValue,
    pub icon: String,
    pub category: Option<I18nValue>,
    pub description: Option<I18nValue>,
    pub output_schema: HashMap<String, String>,
    pub input_schema: HashMap<String, String>,
}

pub trait NodeDefine: Send + Sync {
    fn action_type(&self) -> String;

    fn name(&self) -> I18nValue;

    fn icon(&self) -> String;

    fn category(&self) -> Option<I18nValue>;

    fn description(&self) -> Option<I18nValue>;

    fn output_schema(&self) -> HashMap<String, String>;

    fn input_schema(&self) -> HashMap<String, String>;
}

#[async_trait::async_trait]
pub trait NodeRunner: Send + Sync {
    async fn run(&self, ctx: &Context, param: serde_json::Value) -> Result<(), String>;
}

pub trait NodeRunnerFactory: Send + Sync {
    fn create(&self) -> Box<dyn NodeRunner>;
}
