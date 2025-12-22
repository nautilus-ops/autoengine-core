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
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLW1vdXNlLXBvaW50ZXItY2xpY2staWNvbiBsdWNpZGUtbW91c2UtcG9pbnRlci1jbGljayI+PHBhdGggZD0iTTE0IDQuMSAxMiA2Ii8+PHBhdGggZD0ibTUuMSA4LTIuOS0uOCIvPjxwYXRoIGQ9Im02IDEyLTEuOSAyIi8+PHBhdGggZD0iTTcuMiAyLjIgOCA1LjEiLz48cGF0aCBkPSJNOS4wMzcgOS42OWEuNDk4LjQ5OCAwIDAgMSAuNjUzLS42NTNsMTEgNC41YS41LjUgMCAwIDEtLjA3NC45NDlsLTQuMzQ5IDEuMDQxYTEgMSAwIDAgMC0uNzQuNzM5bC0xLjA0IDQuMzVhLjUuNSAwIDAgMS0uOTUuMDc0eiIvPjwvc3ZnPg==",
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
