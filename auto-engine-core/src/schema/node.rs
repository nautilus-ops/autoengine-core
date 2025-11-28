use std::collections::HashMap;

use schemars::Schema;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::types::MetaData;

type NodeParameters = HashMap<Value, Value>;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NodeSchema {
    pub action_type: String,
    #[serde(flatten)]
    pub metadata: MetaData,
    pub params: Option<NodeParameters>,
    pub icon: Option<String>,

    #[serde(skip)]
    pub position: Position,
    #[serde(skip)]
    pub output_schema: Option<Schema>,
    #[serde(skip)]
    pub input_schema: Option<Schema>,
}

#[cfg(test)]
mod test {
    use crate::schema::node::NodeSchema;

    #[test]
    pub fn test_deserialize() {
        let yaml_str = r#"
action_type: Image
name: 图像识别
params:
  images:
    - a.png
            "#;
        let node: NodeSchema = serde_yaml::from_str(yaml_str).unwrap();

        println!("node schema {:?}", node);
    }
}
