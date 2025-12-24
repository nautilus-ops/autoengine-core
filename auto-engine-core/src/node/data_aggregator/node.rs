use crate::types::field::{FieldType, SchemaField};
use crate::types::node::{I18nValue, NodeDefine};
use std::collections::HashMap;

pub const NODE_TYPE: &str = "DataAggregator";

#[derive(Default)]
pub struct DataAggregatorNode;

impl DataAggregatorNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeDefine for DataAggregatorNode {
    fn action_type(&self) -> String {
        NODE_TYPE.to_string()
    }

    fn name(&self) -> I18nValue {
        I18nValue {
            zh: "数据聚合器".to_string(),
            en: "Data Aggregator".to_string(),
        }
    }

    fn icon(&self) -> String {
        String::from(
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgY2xhc3M9Imx1Y2lkZSBsdWNpZGUtYmV0d2Vlbi1ob3Jpem9udGFsLXN0YXJ0LWljb24gbHVjaWRlLWJldHdlZW4taG9yaXpvbnRhbC1zdGFydCI+PHJlY3Qgd2lkdGg9IjEzIiBoZWlnaHQ9IjciIHg9IjgiIHk9IjMiIHJ4PSIxIi8+PHBhdGggZD0ibTIgOSAzIDMtMyAzIi8+PHJlY3Qgd2lkdGg9IjEzIiBoZWlnaHQ9IjciIHg9IjgiIHk9IjE0IiByeD0iMSIvPjwvc3ZnPg==",
        )
    }

    fn category(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: "数据处理".to_string(),
            en: "Data Processing".to_string(),
        })
    }

    fn description(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: "聚合多个数据源的值到一个对象或数组中".to_string(),
            en: "Aggregate values from multiple data sources into an object or array".to_string(),
        })
    }

    fn output_schema(&self, _input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
        vec![
            SchemaField {
                name: "result".to_string(),
                field_type: FieldType::Object,
                item_type: None,
                description: Some(I18nValue {
                    zh: "聚合后的结果数据".to_string(),
                    en: "Aggregated result data".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
            SchemaField {
                name: "count".to_string(),
                field_type: FieldType::Number,
                item_type: None,
                description: Some(I18nValue {
                    zh: "聚合的数据项数量".to_string(),
                    en: "Number of aggregated data items".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
        ]
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        vec![
            SchemaField {
                name: "mode".to_string(),
                field_type: FieldType::String,
                item_type: None,
                description: Some(I18nValue {
                    zh: "聚合模式：object（对象）或 array（数组）".to_string(),
                    en: "Aggregation mode: object or array".to_string(),
                }),
                enums: vec!["object".to_string(), "array".to_string()],
                default: Some("object".to_string()),
                condition: None,
            },
            SchemaField {
                name: "sources".to_string(),
                field_type: FieldType::Array,
                item_type: Some(FieldType::String),
                description: Some(I18nValue {
                    zh: "数据源路径列表，例如：ctx.node1.value, ctx.node2.result".to_string(),
                    en: "List of data source paths, e.g.: ctx.node1.value, ctx.node2.result"
                        .to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
            SchemaField {
                name: "keys".to_string(),
                field_type: FieldType::Array,
                item_type: Some(FieldType::String),
                description: Some(I18nValue {
                    zh: "对象模式下使用的键名列表（可选，默认使用索引）".to_string(),
                    en: "List of keys for object mode (optional, defaults to indices)".to_string(),
                }),
                enums: vec![],
                default: None,
                condition: None,
            },
        ]
    }
}
