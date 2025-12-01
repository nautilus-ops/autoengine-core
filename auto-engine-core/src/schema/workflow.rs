use serde::{Deserialize, Serialize};

use crate::schema::node::NodeSchema;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkflowSchema {
    pub nodes: Vec<NodeSchema>,
    pub connections: Vec<Connection>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Connection {
    pub from: String,
    pub to: String,
}

#[cfg(test)]
mod test {
    use crate::schema::workflow::WorkflowSchema;

    #[test]
    pub fn test_deserialize() {
        let yaml_str = r#"
nodes:
  - action_type: Image
    name: 图像识别
    params:
      images:
        - a.png
  - action_type: MouseClick
    name: 鼠标点击
    params:
      images:
        - a.png
connections: [
    {
        "from": ""
    }
]
            "#;
        let node: WorkflowSchema = serde_yaml::from_str(yaml_str).unwrap();

        println!("node schema {:?}", node);
    }
}
