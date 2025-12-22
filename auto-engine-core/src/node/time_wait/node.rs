use crate::types::field::{FieldType, SchemaField};
use crate::types::node::{I18nValue, NodeDefine};
use std::collections::HashMap;

#[derive(Default)]
pub struct TimeWaitNode;

impl TimeWaitNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeDefine for TimeWaitNode {
    fn action_type(&self) -> String {
        String::from("TimeWait")
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: "等待时间".to_string(),
            en: "Time Wait".to_string(),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLWNsb2NrLWljb24gbHVjaWRlLWNsb2NrIj48cGF0aCBkPSJNMTIgNnY2bDQgMiIvPjxjaXJjbGUgY3g9IjEyIiBjeT0iMTIiIHI9IjEwIi8+PC9zdmc+",
        )
    }

    fn category(&self) -> Option<I18nValue> {
        Option::from(I18nValue {
            zh: "基础节点".to_string(),
            en: "Basic Node".to_string(),
        })
    }

    fn description(&self) -> Option<I18nValue> {
        Option::from(I18nValue {
            zh: "等待一段时间".to_string(),
            en: "Wait for a while".to_string(),
        })
    }

    fn output_schema(&self, _input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        vec![]
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![SchemaField {
            name: "duration".to_string(),
            field_type: FieldType::Number,
            item_type: None,
            description: Some(I18nValue {
                zh: "需要等待的时间".to_string(),
                en: "The time required to wait".to_string(),
            }),
            enums: vec![],
            default: None,
            condition: None,
        }]
    }
}
