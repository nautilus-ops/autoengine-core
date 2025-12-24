use crate::types::field::{FieldType, SchemaField};
use crate::types::node::{I18nValue, NodeDefine};
use std::collections::HashMap;

#[derive(Default)]
pub struct MouseClickNode {}

impl MouseClickNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeDefine for MouseClickNode {
    fn action_type(&self) -> String {
        String::from("MouseClick")
    }

    fn name(&self) -> crate::types::node::I18nValue {
        I18nValue {
            zh: String::from("鼠标点击"),
            en: String::from("Mouse Click"),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgY2xhc3M9Imx1Y2lkZSBsdWNpZGUtbW91c2UtcG9pbnRlci1jbGljay1pY29uIGx1Y2lkZS1tb3VzZS1wb2ludGVyLWNsaWNrIj48cGF0aCBkPSJNMTQgNC4xIDEyIDYiLz48cGF0aCBkPSJtNS4xIDgtMi45LS44Ii8+PHBhdGggZD0ibTYgMTItMS45IDIiLz48cGF0aCBkPSJNNy4yIDIuMiA4IDUuMSIvPjxwYXRoIGQ9Ik05LjAzNyA5LjY5YS40OTguNDk4IDAgMCAxIC42NTMtLjY1M2wxMSA0LjVhLjUuNSAwIDAgMS0uMDc0Ljk0OWwtNC4zNDkgMS4wNDFhMSAxIDAgMCAwLS43NC43MzlsLTEuMDQgNC4zNWEuNS41IDAgMCAxLS45NS4wNzR6Ii8+PC9zdmc+",
        )
    }

    fn category(&self) -> Option<crate::types::node::I18nValue> {
        Some(I18nValue {
            zh: String::from("桌面自动化"),
            en: String::from("Desktop Automatic"),
        })
    }

    fn description(&self) -> Option<crate::types::node::I18nValue> {
        Some(I18nValue {
            zh: String::from("模拟鼠标点击操作"),
            en: String::from("Simulate mouse click operations"),
        })
    }

    fn output_schema(&self, _input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        Default::default()
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![SchemaField {
            name: "value".to_owned(),
            field_type: FieldType::String,
            item_type: None,
            description: Some(I18nValue {
                zh: "鼠标点击值，示例：left/right".to_owned(),
                en: "Mouse click value, e.g. left/right".to_owned(),
            }),
            enums: vec!["left".to_string(), "right".to_string()],
            default: Some("left".to_string()),
            condition: None,
        }]
    }
}
