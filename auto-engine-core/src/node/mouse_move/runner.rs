use enigo::{Coordinate, Enigo, Mouse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::types::node::NodeRunnerFactory;
use crate::{context::Context, types::node::NodeRunner, utils::parse_variables};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MouseMoveParams {
    pub x: String,
    pub y: String,
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
    async fn run(&mut self, ctx: &Context, values: serde_json::Value) -> Result<(), String> {
        let params: MouseMoveParams = serde_json::from_value(values).map_err(|e| e.to_string())?;
        let x: i32 = parse_variables(ctx, &params.x)
            .await
            .parse()
            .map_err(|e| format!("Failed to parse {} to f64, error: {}", params.x, e))?;

        let y: i32 = parse_variables(ctx, &params.y)
            .await
            .parse()
            .map_err(|e| format!("Failed to parse {} to f64, error: {}", params.y, e))?;

        let rate = ctx.screen_scale;

        let mut enigo = self
            .enigo
            .lock()
            .map_err(|e| format!("Failed to lock the enigo: {}", e))?;

        enigo
            .move_mouse(x / rate as i32, y / rate as i32, Coordinate::Abs)
            .map_err(|err| format!("Failed to move_mouse: {err}"))?;
        Ok(())
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
    fn create(&self) -> Box<dyn NodeRunner> {
        Box::new(MouseMoveRunner::new())
    }
}
