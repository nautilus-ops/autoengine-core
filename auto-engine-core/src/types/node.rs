use crate::context::Context;
use crate::utils;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    Image,
    File,
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
pub trait NodeRunnerControl: Send + Sync {
    async fn run(
        &mut self,
        ctx: &Context,
        node_name: &str,
        params: HashMap<String, serde_json::Value>,
        schema_field: Vec<SchemaField>,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String>;
}

pub struct NodeRunnerController<T: NodeRunner> {
    runner: T,
}

impl<T: NodeRunner> NodeRunnerController<T> {
    pub fn new(runner: T) -> Self {
        Self { runner }
    }
}

#[async_trait::async_trait]
impl<T> NodeRunnerControl for NodeRunnerController<T>
where
    T: NodeRunner,
{
    async fn run(
        &mut self,
        ctx: &Context,
        node_name: &str,
        params: HashMap<String, serde_json::Value>,
        schema_field: Vec<SchemaField>,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let mut params = params;
        for field in schema_field.iter() {
            let default = field.default.clone().unwrap_or_default();
            let mut val = params
                .get(&field.name)
                .unwrap_or(&serde_json::Value::String(default))
                .clone();

            if let serde_json::Value::String(s) = &val {
                let res = utils::parse_variables(ctx, &s).await;
                val = match field.field_type {
                    FieldType::String | FieldType::Image | FieldType::File => {
                        serde_json::Value::String(res.clone())
                    }
                    FieldType::Number => {
                        if let Ok(i) = res.parse::<i64>() {
                            serde_json::Value::Number(i.into())
                        } else if let Ok(f) = res.parse::<f64>() {
                            serde_json::Number::from_f64(f)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null)
                        } else {
                            return Err(format!("Field '{}' cannot be parsed as a number: {}", field.name, res));
                        }
                    }
                    FieldType::Boolean => {
                        match res.to_lowercase().as_str() {
                            "true" | "1" => serde_json::Value::Bool(true),
                            "false" | "0" => serde_json::Value::Bool(false),
                            _ => return Err(format!("Field '{}' cannot be parsed as a boolean: {}", field.name, res)),
                        }
                    },
                    FieldType::Array => {
                        return Err(format!(
                            "Field '{}' cannot be parsed as an array: {}",
                            field.name, res
                        ));
                    }
                    FieldType::Object => {
                        return Err(format!(
                            "Field '{}' cannot be parsed as an object: {}",
                            field.name, res
                        ));
                    }
                };
            }

            params.insert(field.name.clone(), val);
        }

        let params: T::ParamType = serde_json::from_value(serde_json::Value::Object(
            serde_json::map::Map::from_iter(params),
        ))
        .map_err(|e| format!("Failed to parse node parameters: {}", e))?;

        if let Some(result) = self.runner.run(ctx, params).await? {
            for (name,value) in result.iter() {
                ctx.set_value(format!("ctx.{}.{}",node_name, name).as_str(), value).await?;
            }
            return Ok(Some(result))
        }
        Ok(None)
    }
}

#[async_trait::async_trait]
pub trait NodeRunner: Send + Sync {
    type ParamType: Serialize + DeserializeOwned + Send + Sync;
    async fn run(
        &mut self,
        ctx: &Context,
        param: Self::ParamType,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String>;
}

pub trait NodeRunnerFactory: Send + Sync {
    fn create(&self) -> Box<dyn NodeRunnerControl>;
}
