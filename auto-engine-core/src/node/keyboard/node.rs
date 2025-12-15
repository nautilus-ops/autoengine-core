use std::collections::HashMap;
use crate::types::node::{FieldType, I18nValue, NodeDefine, SchemaField};

#[derive(Default)]
pub struct KeyboardNode {}

impl KeyboardNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeDefine for KeyboardNode {
    fn action_type(&self) -> String {
        "KeyBoard".to_string()
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: "键盘".to_string(),
            en: "Keyboard".to_string(),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLWtleWJvYXJkIj48cmVjdCB4PSIyIiB5PSI2IiB3aWR0aD0iMjAiIGhlaWdodD0iMTIiIHJ4PSIyIiByeT0iMiIvPjxwYXRoIGQ9Ik02IDEwaC4wMU0xMCAxMGguMDFNMTQgMTBoLjAxTTE4IDEwaC4wMU02IDE0aDEyIi8+PC9zdmc+",
        )
    }

    fn category(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: "桌面自动化".to_string(),
            en: "Desktop Automatic".to_string(),
        })
    }

    fn description(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: "模拟键盘按键或文本输入".to_string(),
            en: "Simulate keyboard key presses or text input".to_string(),
        })
    }

    fn output_schema(&self,_input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        Default::default()
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![
            SchemaField {
                name: "mode".to_owned(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "键盘操作模式，支持点击、按下、抬起或文本输入".to_owned(),
                    en: "Keyboard mode: click, press, release, or type text".to_owned(),
                }),
                enums: vec![
                    "Click".to_string(),
                    "Down".to_string(),
                    "Up".to_string(),
                    "Type".to_string(),
                ],
                default: Some("Click".to_string()),
            },
            SchemaField {
                name: "key".to_owned(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "键盘按键，示例：A、Enter、F1、Control 等".to_owned(),
                    en: "Keyboard key, e.g. A, Enter, F1, Control".to_owned(),
                }),
                enums: vec![],
                default: None,
            },
            SchemaField {
                name: "value".to_owned(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "文本输入内容，仅在 Type 模式下使用".to_owned(),
                    en: "Text to input when mode is Type".to_owned(),
                }),
                enums: vec![],
                default: None,
            },
        ]
    }
}
