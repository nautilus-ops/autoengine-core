use enigo::{Button, Enigo, Mouse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{
    context::Context,
    types::node::{NodeRunner, NodeRunnerFactory},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MouseClickParams {
    pub value: String,
}

#[derive(Clone)]
pub struct MouseClickRunner {
    enigo: Arc<Mutex<Enigo>>,
}

impl MouseClickRunner {
    fn new() -> Self {
        let enigo = Enigo::new(&Default::default())
            .map_err(|e| e.to_string())
            .unwrap();
        Self {
            enigo: Arc::new(Mutex::new(enigo)),
        }
    }
}

#[async_trait::async_trait]
impl NodeRunner for MouseClickRunner {
    async fn run(&mut self, _ctx: &Context, values: serde_json::Value) -> Result<(), String> {
        let params: MouseClickParams = serde_json::from_value(values).map_err(|e| e.to_string())?;

        let mut enigo = self
            .enigo
            .lock()
            .map_err(|e| format!("Failed to lock the enigo: {}", e))?;

        let btn = match params.value.to_lowercase().as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            _ => {
                return Err(format!("Invalid button value '{}'", params.value));
            }
        };

        enigo
            .button(btn, enigo::Direction::Click)
            .map_err(|err| format!("Failed to click {}: {err}", params.value))?;
        Ok(())
    }
}

#[derive(Default)]
pub struct MouseClickNodeFactory;

impl MouseClickNodeFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeRunnerFactory for MouseClickNodeFactory {
    fn create(&self) -> Box<dyn NodeRunner> {
        Box::new(MouseClickRunner::new())
    }
}
