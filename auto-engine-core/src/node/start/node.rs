use crate::types::node::{FieldType, I18nValue, NodeDefine, SchemaField};
use std::collections::HashMap;

pub struct StartNode;

impl StartNode {
    pub(crate) fn new() -> StartNode {
        Self {}
    }
}

impl NodeDefine for StartNode {
    fn action_type(&self) -> String {
        String::from("Start")
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: String::from("开始"),
            en: String::from("Start"),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLWNpcmNsZS1wbGF5LWljb24gbHVjaWRlLWNpcmNsZS1wbGF5Ij48cGF0aCBkPSJNOSA5LjAwM2ExIDEgMCAwIDEgMS41MTctLjg1OWw0Ljk5NyAyLjk5N2ExIDEgMCAwIDEgMCAxLjcxOGwtNC45OTcgMi45OTdBMSAxIDAgMCAxIDkgMTQuOTk2eiIvPjxjaXJjbGUgY3g9IjEyIiBjeT0iMTIiIHI9IjEwIi8+PC9zdmc+",
        )
    }

    fn category(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: String::from("基础节点"),
            en: String::from("Basic Node"),
        })
    }

    fn description(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: String::from("工作流从此节点开始执行"),
            en: String::from("The workflow start at this node."),
        })
    }

    fn output_schema(&self, input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        let val = input.get("params").unwrap_or_default().clone();
        let params: HashMap<String, serde_json::Value> =
            serde_json::from_value(val).unwrap_or_default();

        let mut outputs = vec![];
        for (key, _value) in params.iter() {
            outputs.push(SchemaField {
                name: key.to_string(),
                field_type: Default::default(),
                item_type: None,
                description: None,
                enums: vec![],
                default: None,
            });
        }

        outputs
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![SchemaField {
            name: "params".to_string(),
            field_type: FieldType::Object,
            item_type: None,
            description: Some(I18nValue {
                zh: "".to_string(),
                en: "".to_string(),
            }),
            enums: vec![],
            default: None,
        }]
    }
}
