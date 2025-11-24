use crate::converter::types::keymousego::{Event, KeyActionParam, Script};
use auto_engine_core::types::{
    KeyBoardParams, MetaData, Node, Pipeline, Stage, ToKeyCode, ToKeyMode,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub enum ConverterFrom {
    KeyMouseGo,
}
pub struct Converter {
    content: String,
}
impl Converter {
    pub fn new(config_path: &PathBuf) -> Converter {
        let content = fs::read_to_string(config_path).unwrap();
        Converter { content }
    }

    pub fn convert(&self, config: ConverterFrom) -> Result<Pipeline, Box<dyn Error>> {
        let mut pipelines = Pipeline::new();
        match config {
            ConverterFrom::KeyMouseGo => {
                let script: Script = json5::from_str(&self.content).unwrap();

                for (index, event) in script.scripts.iter().enumerate() {
                    match &event {
                        Event::EK {
                            delay,
                            action_type,
                            action,
                        } => {
                            let param = action.clone();
                            let key = match param {
                                KeyActionParam::Key(values) => values
                                    .get(1)
                                    .unwrap()
                                    .clone()
                                    .to_string()
                                    .trim()
                                    .trim_matches('"')
                                    .to_owned(),
                            };

                            if *delay > 10 {
                                let node = Node::TimeWait {
                                    metadata: MetaData {
                                        name: format!("wait-{}-{}-{}", action_type, key, index),
                                        description: None,
                                        duration: Some(*delay as u32),
                                        retry: None,
                                        interval: None,
                                        conditions: None,
                                        err_return: None,
                                    },
                                };
                                pipelines.push(Stage { stage: vec![node] })
                            }

                            let node = Node::KeyBoard {
                                metadata: MetaData {
                                    name: format!("{}-{}-{}", action_type, key, index),
                                    duration: None,
                                    retry: Some(0),
                                    interval: Some(0),
                                    conditions: None,
                                    err_return: None,
                                    description: None,
                                },
                                params: KeyBoardParams {
                                    mode: action_type.to_key_mode(),
                                    key: key.to_key_code().unwrap_or_else(|| {
                                        panic!(
                                            "failed to convert key {} {:?} to KeyCode",
                                            key.trim().len(),
                                            key.trim().as_bytes()
                                        )
                                    }),
                                    value: None,
                                },
                            };

                            pipelines.push(Stage { stage: vec![node] });
                        }
                        _ => {
                            continue;
                        }
                    };
                }
            }
        }

        Ok(pipelines)
    }
}
