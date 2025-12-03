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
            "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLXNwbGluZS1wb2ludGVyLWljb24gbHVjaWRlLXNwbGluZS1wb2ludGVyIj48cGF0aCBkPSJNMTIuMDM0IDEyLjY4MWEuNDk4LjQ5OCAwIDAgMSAuNjQ3LS42NDdsOSAzLjVhLjUuNSAwIDAgMS0uMDMzLjk0M2wtMy40NDQgMS4wNjhhMSAxIDAgMCAwLS42Ni42NmwtMS4wNjcgMy40NDNhLjUuNSAwIDAgMS0uOTQzLjAzM3oiLz48cGF0aCBkPSJNNSAxN0ExMiAxMiAwIDAgMSAxNyA1Ii8+PGNpcmNsZSBjeD0iMTkiIGN5PSI1IiByPSIyIi8+PGNpcmNsZSBjeD0iNSIgY3k9IjE5IiByPSIyIi8+PC9zdmc+",
        )
    }

    fn output_schema(&self) -> HashMap<String, String> {
        Default::default()
    }

    fn input_schema(&self) -> HashMap<String, String> {
        let mut input = HashMap::new();
        input.insert("x".to_owned(), "string".to_owned());
        input.insert("y".to_owned(), "string".to_owned());
        input
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
}
