use crate::context::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct I18nValue {
    pub zh: String,
    pub en: String,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    #[default]
    String,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Clone, Default, Serialize, Debug, Deserialize)]
pub struct SchemaField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct NodeType {
    pub action_type: String,
    pub name: I18nValue,
    pub icon: String,
    pub category: Option<I18nValue>,
    pub description: Option<I18nValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_schema: Vec<SchemaField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_schema: Vec<SchemaField>,
}

pub trait NodeDefine: Send + Sync {
    fn action_type(&self) -> String;

    fn name(&self) -> I18nValue;

    fn icon(&self) -> String;

    fn category(&self) -> Option<I18nValue>;

    fn description(&self) -> Option<I18nValue>;

    fn output_schema(&self) -> Vec<SchemaField>;

    fn input_schema(&self) -> Vec<SchemaField>;
}

#[async_trait::async_trait]
pub trait NodeRunner: Send + Sync {
    async fn run(&self, ctx: &Context, param: serde_json::Value) -> Result<(), String>;
}

pub trait NodeRunnerFactory: Send + Sync {
    fn create(&self) -> Box<dyn NodeRunner>;
}
