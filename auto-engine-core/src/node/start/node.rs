use crate::types::node::{I18nValue, NodeDefine, SchemaField};

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

    fn output_schema(&self) -> Vec<SchemaField> {
        Default::default()
    }

    fn input_schema(&self) -> Vec<SchemaField> {
        Default::default()
    }

    fn category(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: String::from("基础节点"),
            en: String::from("Base node"),
        })
    }

    fn description(&self) -> Option<I18nValue> {
        Some(I18nValue {
            zh: String::from("工作流从此节点开始执行"),
            en: String::from("The workflow start at this node."),
        })
    }
}
