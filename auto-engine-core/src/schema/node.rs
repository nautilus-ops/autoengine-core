use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::types::{MetaData, node::NodeType};

type NodeParameters = HashMap<Value, Value>;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Position {
    #[serde(default)]
    pub x: i64,
    #[serde(default)]
    pub y: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NodeSchema {
    pub node_id: String,
    pub action_type: String,
    #[serde(flatten)]
    pub metadata: MetaData,
    #[deprecated(since = "0.2.2", note = "Use `input_data` instead")]
    pub params: Option<NodeParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_define: Option<NodeType>,
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
