use serde::{Deserialize, Serialize};

use crate::schema::node::NodeSchema;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct WorkflowSchema {
    pub nodes: Vec<NodeSchema>,
    pub connections: Vec<Connection>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
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
  - node_id: a
    action_type: Image
    name: ImageMatch
    params:
      images:
        - a.png
  - node_id: b
    action_type: MouseClick
    name: MouseClick
    params:
      images:
        - a.png
connections:
  - from: "a"
    to: "b"
            "#;
        let node: WorkflowSchema = serde_yaml::from_str(yaml_str).unwrap();

        println!("node schema {:?}", node);
    }
}
