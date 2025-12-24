use enigo::{Coordinate, Enigo, Mouse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::types::node::{NodeRunnerControl, NodeRunnerController, NodeRunnerFactory};
use crate::{context::Context, types::node::NodeRunner};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MouseMoveParams {
    pub x: i32,
    pub y: i32,
    pub hidpi: String,
}

#[derive(Clone)]
pub struct MouseMoveRunner {
    enigo: Arc<Mutex<Enigo>>,
}

impl MouseMoveRunner {
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
impl NodeRunner for MouseMoveRunner {
    type ParamType = MouseMoveParams;

    async fn run(
        &mut self,
        _ctx: &Context,
        params: Self::ParamType,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let rate: f64 = match params.hidpi.as_str() {
            "100%" => 1.0,
            "200%" => 2.0,
            "400%" => 4.0,
            _ => 1.0,
        };

        let x: i32 = params.x;

        let y: i32 = params.y;

        let mut enigo = self
            .enigo
            .lock()
            .map_err(|e| format!("Failed to lock the enigo: {}", e))?;

        enigo
            .move_mouse(x / rate as i32, y / rate as i32, Coordinate::Abs)
            .map_err(|err| format!("Failed to move_mouse: {err}"))?;
        Ok(None)
    }
}

#[derive(Default)]
pub struct MouseMoveNodeFactory;

impl MouseMoveNodeFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeRunnerFactory for MouseMoveNodeFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(MouseMoveRunner::new()))
    }
}
