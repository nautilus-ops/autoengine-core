use crate::node_register::host;
use crate::types::node::{I18nValue, NodeDefine};
use std::collections::HashMap;

pub struct WasmNode {
    action_type: String,
    node_name: I18nValue,
    icon: String,
    output_schema: String,
    input_schema: String,
    description: I18nValue,
    category: I18nValue,
}

impl WasmNode {
    pub fn from_node(n: host::Node) -> Self {
        Self {
            action_type: n.action_type,
            node_name: I18nValue {
                en: n.name.en,
                zh: n.name.zh,
            },
            icon: n.icon,
            output_schema: n.output_schema,
            input_schema: n.input_schema,
            description: I18nValue {
                zh: n.description.zh,
                en: n.description.en,
            },
            category: I18nValue {
                zh: n.category.zh,
                en: n.category.en,
            },
        }
    }
}

impl NodeDefine for WasmNode {
    fn action_type(&self) -> String {
        self.action_type.clone()
    }

    fn name(&self) -> I18nValue {
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

    fn category(&self) -> Option<I18nValue> {
        Some(self.category.clone())
    }

    fn description(&self) -> Option<I18nValue> {
        Some(self.description.clone())
    }
}
