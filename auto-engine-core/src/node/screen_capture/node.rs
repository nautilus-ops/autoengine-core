use crate::types::field::{
    Condition, FieldCondition, FieldType, SchemaField, StringConstraint, ValueConstraint,
};
use crate::types::node::{I18nValue, NodeDefine};
use std::collections::HashMap;

pub const NODE_TYPE: &str = "ScreenCapture";

#[derive(Default)]
pub struct ScreenCaptureNode;

impl ScreenCaptureNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeDefine for ScreenCaptureNode {
    fn action_type(&self) -> String {
        NODE_TYPE.to_string()
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: "屏幕截图".to_string(),
            en: "Screen Capture".to_string(),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiPjxyZWN0IHg9IjMiIHk9IjQiIHdpZHRoPSIxOCIgaGVpZ2h0PSIxMiIgcng9IjIiIHJ5PSIyIi8+PHBhdGggZD0iTTkgMjBoNiIvPjxwYXRoIGQ9Ik0xMiAxNnY0Ii8+PGNpcmNsZSBjeD0iMTIiIGN5PSIxMCIgcj0iMi41Ii8+PC9zdmc+",
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
            zh: "截取桌面屏幕，可选择全屏或指定区域。".to_string(),
            en: "Capture desktop screen as a full image or specified area and save to file."
                .to_string(),
        })
    }

    fn output_schema(&self, _input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        vec![
            SchemaField {
                name: "file".to_string(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "截图保存名称".to_string(),
                    en: "Saved screenshot name".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
            SchemaField {
                name: "width".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "截图宽度".to_string(),
                    en: "Screenshot width".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
            SchemaField {
                name: "height".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "截图高度".to_string(),
                    en: "Screenshot height".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
        ]
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        let area_condition = Condition::Field(FieldCondition {
            field: "mode".to_string(),
            constraint: ValueConstraint::String(StringConstraint {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
                equals: Some("area".to_string()),
                enum_values: None,
            }),
            required: true,
        });
        vec![
            SchemaField {
                name: "mode".to_string(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "截图模式：full/area".to_string(),
                    en: "Capture mode: full or area".to_string(),
                }),
                enums: vec!["full".to_string(), "area".to_string()],
                default: Some("full".to_string()),
                condition: None,
            },
            SchemaField {
                name: "file_name".to_string(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "保存的文件名。".to_string(),
                    en: "File name to save under current workflow directory.".to_string(),
                }),
                enums: vec![],
                default: Some("screenshot.png".to_string()),
                condition: None,
            },
            SchemaField {
                name: "screen_index".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "使用的屏幕索引，默认0。".to_string(),
                    en: "Screen index to capture, default 0.".to_string(),
                }),
                enums: vec![],
                default: Some("0".to_string()),
                condition: None,
            },
            SchemaField {
                name: "x".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "区域截图的起始X坐标".to_string(),
                    en: "Start X for area capture".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: Some(area_condition.clone()),
            },
            SchemaField {
                name: "y".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "区域截图的起始Y坐标".to_string(),
                    en: "Start Y for area capture".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: Some(area_condition.clone()),
            },
            SchemaField {
                name: "width".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "区域截图的宽度".to_string(),
                    en: "Width for area capture".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: Some(area_condition.clone()),
            },
            SchemaField {
                name: "height".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "区域截图的高度".to_string(),
                    en: "Height for area capture".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: Some(area_condition),
            },
        ]
    }
}
