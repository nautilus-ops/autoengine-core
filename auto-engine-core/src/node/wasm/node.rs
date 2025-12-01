use std::collections::HashMap;
use crate::node_register::host;
use crate::types::node::{NodeDefine, NodeName};

pub struct WasmNode {
    action_type: String,
    node_name: NodeName,
    icon: String,
    output_schema: String,
    input_schema: String,
}

impl WasmNode {
    pub fn from_node(n: host::Node) -> Self {
        Self {
            action_type: n.action_type,
            node_name: NodeName {
                en: n.name.en,
                zh: n.name.zh,
            },
            icon: n.icon,
            output_schema: n.output_schema,
            input_schema: n.input_schema,
        }
    }
}

impl NodeDefine for WasmNode {
    fn action_type(&self) -> String {
        self.action_type.clone()
    }

    fn name(&self) -> NodeName {
        self.node_name.clone()
    }

    fn icon(&self) -> String {
        self.icon.clone()
    }

    fn output_schema(&self) -> HashMap<String, String> {
        serde_json::from_str(&self.output_schema).unwrap()
    }

    fn input_schema(&self) -> HashMap<String, String> {
        serde_json::from_str(&self.input_schema).unwrap()
    }
}
