use crate::types::node::{FieldType, I18nValue, NodeDefine, SchemaField};
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
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiPjxwYXRoIGQ9Ik0yMSAxNlY4YTIgMiAwIDAgMC0xLTEuNzNsLTctNGEyIDIgMCAwIDAtMiAwbC03IDRBMiAyIDAgMCAwIDMgOHY4YTIgMiAwIDAgMCAxIDEuNzNsNyA0YTIgMiAwIDAgMCAyIDBsNy00QTIgMiAwIDAgMCAyMSAxNnoiLz48cG9seWxpbmUgcG9pbnRzPSI3LjUgNC4yMSAxMiA2LjgxIDE2LjUgNC4yMSIvPjxwb2x5bGluZSBwb2ludHM9IjcuNSAxOS43OSA3LjUgMTQuNiAzIDE3LjQiLz48cG9seWxpbmUgcG9pbnRzPSIyMSAxMi4yIDIxIDE2Ljc5IDE2LjUgMTkuNzkgMTYuNSAxNC42Ii8+PHBvbHlsaW5lIHBvaW50cz0iMy4yNyA2Ljk2IDEyIDEyLjAxIDIwLjczIDYuOTYiLz48bGluZSB4MT0iMTIiIHgyPSIxMiIgeTE9IjIyLjA4IiB5Mj0iMTIiLz48L3N2Zz4=",
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

    fn output_schema(&self,_input: HashMap<String, serde_json::Value>) -> Vec<SchemaField> {
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
            },
        ]
    }
}
