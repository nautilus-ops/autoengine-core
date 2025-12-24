use crate::types::field::{FieldType, SchemaField};
use crate::types::node::{I18nValue, NodeDefine};
use std::collections::HashMap;

#[derive(Default)]
pub struct MouseMoveNode {}

impl MouseMoveNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeDefine for MouseMoveNode {
    fn action_type(&self) -> String {
        "MouseMove".to_string()
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: "鼠标移动".to_string(),
            en: "MouseMove".to_string(),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgY2xhc3M9Imx1Y2lkZSBsdWNpZGUtc3BsaW5lLXBvaW50ZXItaWNvbiBsdWNpZGUtc3BsaW5lLXBvaW50ZXIiPjxwYXRoIGQ9Ik0xMi4wMzQgMTIuNjgxYS40OTguNDk4IDAgMCAxIC42NDctLjY0N2w5IDMuNWEuNS41IDAgMCAxLS4wMzMuOTQzbC0zLjQ0NCAxLjA2OGExIDEgMCAwIDAtLjY2LjY2bC0xLjA2NyAzLjQ0M2EuNS41IDAgMCAxLS45NDMuMDMzeiIvPjxwYXRoIGQ9Ik01IDE3QTEyIDEyIDAgMCAxIDE3IDUiLz48Y2lyY2xlIGN4PSIxOSIgY3k9IjUiIHI9IjIiLz48Y2lyY2xlIGN4PSI1IiBjeT0iMTkiIHI9IjIiLz48L3N2Zz4=",
        )
    }

    fn category(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: String::from("桌面自动化"),
            en: String::from("Desktop Automatic"),
        })
    }

    fn description(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: String::from("接收参数x和y坐标，模拟鼠标移动"),
            en: String::from(
                "Accepts x and y coordinates as parameters to simulate mouse movement.",
            ),
        })
    }

    fn output_schema(&self, _input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        Default::default()
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![
            SchemaField {
                name: "x".to_owned(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "鼠标移动的横坐标".to_owned(),
                    en: "Horizontal position of mouse move".to_owned(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
            SchemaField {
                name: "y".to_owned(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "鼠标移动的纵坐标".to_owned(),
                    en: "Vertical position of mouse move".to_owned(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
            SchemaField {
                name: "hidpi".to_owned(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "HiDPI ".to_owned(),
                    en: "HiDPI".to_owned(),
                }),
                enums: vec!["100%".to_string(), "200%".to_string(), "400%".to_string()],
                default: Some("100%".to_string()),
                condition: None,
            },
        ]
    }
}
