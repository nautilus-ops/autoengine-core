use crate::types::node::{FieldType, I18nValue, NodeDefine, SchemaField};

#[derive(Default)]
pub struct ImageMatchNode;

impl ImageMatchNode {
    pub fn new() -> Self {
        ImageMatchNode {}
    }
}

impl NodeDefine for ImageMatchNode {
    fn action_type(&self) -> String {
        String::from("ImageMatch")
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: "图像匹配".to_string(),
            en: "Image Match".to_string(),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLWltYWdlcy1pY29uIGx1Y2lkZS1pbWFnZXMiPjxwYXRoIGQ9Im0yMiAxMS0xLjI5Ni0xLjI5NmEyLjQgMi40IDAgMCAwLTMuNDA4IDBMMTEgMTYiLz48cGF0aCBkPSJNNCA4YTIgMiAwIDAgMC0yIDJ2MTBhMiAyIDAgMCAwIDIgMmgxMGEyIDIgMCAwIDAgMi0yIi8+PGNpcmNsZSBjeD0iMTMiIGN5PSI3IiByPSIxIiBmaWxsPSJjdXJyZW50Q29sb3IiLz48cmVjdCB4PSI4IiB5PSIyIiB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHJ4PSIyIi8+PC9zdmc+",
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
            zh: "找到目标图片在模板图片上的坐标位置".to_string(),
            en: "Determine the coordinates of the target image on the template image".to_string(),
        })
    }

    fn output_schema(&self) -> Vec<SchemaField> {
        vec![SchemaField {
            name: "score".to_string(),
            field_type: FieldType::Number,
            item_type: None,
            description: Some(I18nValue {
                zh: "匹配结果分值，最小为0，最大为1".to_string(),
                en: "Matching final score, minimum 0, maximum 1".to_string(),
            }),
            enums: vec![],
            default: Some("0.8".to_string()),
        }]
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![
            SchemaField {
                name: "target_score".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "目标匹配分值，最小为0，最大为1".to_string(),
                    en: "Target matching score, minimum 0, maximum 1".to_string(),
                }),
                enums: vec![],
                default: Some("0.8".to_string()),
            },
            SchemaField {
                name: "imread_type".to_string(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "图像读取方式".to_string(),
                    en: "Image read mode".to_string(),
                }),
                enums: vec!["Grayscale".to_string(), "Color".to_string()],
                default: Some("Grayscale".to_string()),
            },
            SchemaField {
                name: "use_screenshot".to_string(),
                field_type: FieldType::Boolean,
                item_type: None,
                description: Some(I18nValue {
                    zh: "是否使用桌面截图作为源图片？".to_string(),
                    en: "Should desktop screenshots be used as source images?".to_string(),
                }),
                enums: vec![],
                default: None,
            },
            SchemaField {
                name: "resize".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "图片大小倍率，倍率越小匹配速度越快，精度越低".to_string(),
                    en: "Image scaling factor: The smaller the scaling factor, the faster the matching speed but the lower the accuracy.".to_string(),
                }),
                enums: vec![String::from("0.5"), String::from("1"), String::from("2")],
                default: Some(String::from("1")),
            },
            SchemaField {
                name: "template_image".to_string(),
                field_type: FieldType::File,
                item_type: None,
                description: Some(I18nValue {
                    zh: "图像模板".to_string(),
                    en: "Image template".to_string(),
                }),
                enums: vec![],
                default: None,
            },
            SchemaField {
                name: "source_image".to_string(),
                field_type: FieldType::File,
                item_type: None,
                description: Some(I18nValue {
                    zh: "原始图像".to_string(),
                    en: "Source image".to_string(),
                }),
                enums: vec![],
                default: None,
            },
        ]
    }
}
