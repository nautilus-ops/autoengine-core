use crate::converter::types::quickinput;
use crate::converter::types::quickinput::{Action, QuickInputMacro};
use auto_engine_core::types::{
    KeyBoardKeyMode, KeyBoardParams, KeyCode, MetaData, Node, Pipeline, Stage, ToKeyCode,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use auto_engine_core::types::conditions::Conditions;

pub struct Converter {
    content: String,
    conditions: Option<Conditions>,
    pub duration: Option<u32>,
}

impl Converter {
    pub fn new(
        config_path: &PathBuf,
        with_exist: Option<String>,
        with_duration: Option<u32>,
    ) -> Converter {
        let content = fs::read_to_string(config_path).unwrap();
        let conditions = Conditions{
            exist: with_exist,
            condition: None,
            not_exist: None,
        };
        Converter {
            content,
            conditions: Some(conditions),
            duration: with_duration,
        }
    }

    pub fn convert(&self) -> Result<Pipeline, Box<dyn Error>> {
        let mut pipelines = Pipeline::new();
        let quick_input: QuickInputMacro = serde_json::from_str(self.content.as_str())?;

        for (index, action) in quick_input.actions.iter().enumerate() {
            if action.dis {
                continue;
            }
            match action.kind {
                quickinput::ACTION_KEY => {
                    let option = parse_action_params(action);
                    if let Some((params, key)) = option {
                        let node = Node::KeyBoard {
                            metadata: MetaData {
                                name: format!("key-{}-{}", index, key),
                                duration: self.duration.clone(),
                                retry: Some(0),
                                interval: Some(0),
                                conditions: self.conditions.clone(),
                                err_return: None,
                                description: None,
                            },
                            params,
                        };

                        pipelines.push(Stage { stage: vec![node] })
                    } else {
                        continue;
                    }
                }
                quickinput::ACTION_DELAY => {
                    if action.ms.is_none() {
                        continue;
                    }
                    let ms = action.ms.unwrap();

                    let node = Node::TimeWait {
                        metadata: MetaData {
                            name: format!("wait-{}", index),
                            duration: Some(ms as u32),
                            conditions: self.conditions.clone(),
                            description: None,
                            retry: Some(0),
                            interval: Some(0),
                            err_return: None,
                        },
                    };

                    pipelines.push(Stage { stage: vec![node] })
                }
                quickinput::ACTION_END => continue,
                _ => continue,
            }
        }

        Ok(pipelines)
    }
}

pub fn parse_action_params(action: &Action) -> Option<(KeyBoardParams, String)> {
    if action.state.is_none() || action.vk.is_none() {
        return None;
    }

    let key = virtual_key_to_string(action.vk.unwrap());

    let key = key?;

    let params: KeyBoardParams = match action.state.unwrap() {
        // release
        0 => KeyBoardParams {
            mode: KeyBoardKeyMode::Up,
            key: key.to_key_code().unwrap_or(KeyCode::A),
            value: None,
        },
        // press
        1 => KeyBoardParams {
            mode: KeyBoardKeyMode::Down,
            key: key.to_key_code().unwrap_or(KeyCode::A),
            value: None,
        },
        // click
        2 => KeyBoardParams {
            mode: KeyBoardKeyMode::Click,
            key: key.to_key_code().unwrap_or(KeyCode::A),
            value: None,
        },
        _ => return None,
    };

    Some((params, key))
}

/// Converts a Windows-style virtual-key code to a normalized string.
pub fn virtual_key_to_string(vk: i32) -> Option<String> {
    match vk {
        // Alphabetic keys
        code @ 0x41..=0x5A => char::from_u32(code as u32).map(|c| c.to_string()),
        // Number keys (top row)
        code @ 0x30..=0x39 => char::from_u32(code as u32).map(|c| c.to_string()),
        // Function keys
        code @ 0x70..=0x7B => Some(format!("F{}", code - 0x6F)),
        // Numpad digits
        code @ 0x60..=0x69 => Some(format!("Num{}", code - 0x60)),
        0x0D => Some("Enter".into()),
        0x1B => Some("Escape".into()),
        0x08 => Some("Backspace".into()),
        0x09 => Some("Tab".into()),
        0x20 => Some("Space".into()),
        0x10 => Some("Shift".into()),
        0x11 => Some("Control".into()),
        0x12 => Some("Alt".into()),
        0x5B | 0x5C => Some("Win".into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::virtual_key_to_string;

    #[test]
    fn converts_letters() {
        assert_eq!(virtual_key_to_string(0x41).as_deref(), Some("A"));
        assert_eq!(virtual_key_to_string(0x5A).as_deref(), Some("Z"));
    }

    #[test]
    fn converts_digits() {
        assert_eq!(virtual_key_to_string(0x30).as_deref(), Some("0"));
        assert_eq!(virtual_key_to_string(0x39).as_deref(), Some("9"));
    }

    #[test]
    fn converts_function_keys() {
        assert_eq!(virtual_key_to_string(0x70).as_deref(), Some("F1"));
        assert_eq!(virtual_key_to_string(0x7B).as_deref(), Some("F12"));
    }

    #[test]
    fn converts_control_keys() {
        assert_eq!(virtual_key_to_string(0x0D).as_deref(), Some("Enter"));
        assert_eq!(virtual_key_to_string(0x20).as_deref(), Some("Space"));
        assert_eq!(virtual_key_to_string(0x10).as_deref(), Some("Shift"));
    }

    #[test]
    fn converts_numpad_keys() {
        assert_eq!(virtual_key_to_string(0x60).as_deref(), Some("Num0"));
        assert_eq!(virtual_key_to_string(0x69).as_deref(), Some("Num9"));
    }

    #[test]
    fn returns_none_for_unknown_codes() {
        assert_eq!(virtual_key_to_string(0x00), None);
        assert_eq!(virtual_key_to_string(0xFE), None);
    }
}
