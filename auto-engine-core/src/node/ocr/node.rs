use crate::types::field::{FieldType, SchemaField};
use crate::types::node::{I18nValue, NodeDefine};
use std::collections::HashMap;

pub const NODE_TYPE: &str = "OCR";

#[derive(Default)]
pub struct OcrNode;

impl OcrNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeDefine for OcrNode {
    fn action_type(&self) -> String {
        NODE_TYPE.to_string()
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: "本地OCR".to_string(),
            en: "Local OCR".to_string(),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgY2xhc3M9Imx1Y2lkZSBsdWNpZGUtc2Nhbi10ZXh0LWljb24gbHVjaWRlLXNjYW4tdGV4dCI+PHBhdGggZD0iTTMgN1Y1YTIgMiAwIDAgMSAyLTJoMiIvPjxwYXRoIGQ9Ik0xNyAzaDJhMiAyIDAgMCAxIDIgMnYyIi8+PHBhdGggZD0iTTIxIDE3djJhMiAyIDAgMCAxLTIgMmgtMiIvPjxwYXRoIGQ9Ik03IDIxSDVhMiAyIDAgMCAxLTItMnYtMiIvPjxwYXRoIGQ9Ik03IDhoOCIvPjxwYXRoIGQ9Ik03IDEyaDEwIi8+PHBhdGggZD0iTTcgMTZoNiIvPjwvc3ZnPg==",
        )
    }

    fn category(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: "图像处理".to_string(),
            en: "Image Processing".to_string(),
        })
    }

    fn description(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: "对图片执行 OCR 识别，可配置语言、字符白名单及数字模式（需本地安装 Tesseract）。"
                .to_string(),
            en: "Run OCR on an image with language/whitelist/numeric options (requires local Tesseract)."
                .to_string(),
        })
    }

    fn output_schema(&self, _input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        vec![SchemaField {
            name: "text".to_string(),
            field_type: FieldType::String,
            item_type: None,
            description: Some(I18nValue {
                zh: "识别出的文本".to_string(),
                en: "Recognized text".to_string(),
            }),
            enums: vec![],
            default: None,
            condition: None,
        }]
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![
            SchemaField {
                name: "image".to_string(),
                field_type: FieldType::File,
                item_type: None,
                description: Some(I18nValue {
                    zh: "待识别的图片路径，支持绝对路径或相对工作流目录。".to_string(),
                    en: "Image path for OCR, absolute or relative to workflow directory."
                        .to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
            SchemaField {
                name: "language".to_string(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "Tesseract 语言（如 eng, chi_sim）".to_string(),
                    en: "Tesseract language (e.g. eng, chi_sim)".to_string(),
                }),
                enums: vec![],
                default: Some("eng".to_string()),
                condition: None,
            },
            SchemaField {
                name: "whitelist".to_string(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "可选字符白名单，留空表示不限制。".to_string(),
                    en: "Optional character whitelist, empty to disable.".to_string(),
                }),
                enums: vec![],
                default: Some("".to_string()),
                condition: None,
            },
            SchemaField {
                name: "numeric_mode".to_string(),
                field_type: FieldType::Boolean,
                item_type: None,
                description: Some(I18nValue {
                    zh: "启用 Tesseract 数字模式，提高数字识别准确率。".to_string(),
                    en: "Enable Tesseract numeric mode to improve digit recognition.".to_string(),
                }),
                enums: vec![],
                default: Some("false".to_string()),
                condition: None,
            },
            SchemaField {
                name: "digits_only".to_string(),
                field_type: FieldType::Boolean,
                item_type: None,
                description: Some(I18nValue {
                    zh: "仅保留 ASCII 数字，过滤其他字符。".to_string(),
                    en: "Keep ASCII digits only, filter other characters.".to_string(),
                }),
                enums: vec![],
                default: Some("false".to_string()),
                condition: None,
            },
        ]
    }
}
