use std::collections::HashMap;
use enigo::{Direction, Enigo, Keyboard};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{
    context::Context,
    types::{
        KeyBoardKeyMode, ToKeyCode,
        node::{NodeRunner, NodeRunnerFactory},
    },
};
use crate::types::node::{NodeRunnerControl, NodeRunnerController};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct KeyboardParams {
    #[serde(default)]
    pub mode: KeyBoardKeyMode,
    pub key: String,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Clone)]
pub struct KeyboardNodeRunner {
    enigo: Arc<Mutex<Enigo>>,
}

impl KeyboardNodeRunner {
    fn new(enigo: Arc<Mutex<Enigo>>) -> Self {
        Self { enigo: enigo }
    }

    #[cfg(all(target_os = "macos", feature = "tauri"))]
    fn with_enigo<F>(&self, ctx: &Context, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut Enigo) -> Result<(), String> + Send + 'static,
    {
        use std::sync::mpsc;

        let app_handle = ctx
            .app_handle
            .clone()
            .ok_or_else(|| "Tauri app handle is required on macOS".to_string())?;
        let lock = self.enigo.clone();
        let (tx, rx) = mpsc::channel::<Result<(), String>>();

        let result = app_handle.run_on_main_thread(move || {
            let result = (|| {
                let mut enigo = lock
                    .lock()
                    .map_err(|e| format!("Failed to lock the enigo: {}", e))?;
                f(&mut enigo)
            })();

            if let Err(e) = tx.send(result) {
                log::error!("Failed to send result from main thread: {e}");
            }
        });

        if let Err(e) = result {
            return Err(format!("Failed to run on main thread: {e}"));
        }

        rx.recv()
            .unwrap_or_else(|e| Err(format!("Failed to receive result: {e}")))
    }

    #[cfg(not(all(target_os = "macos", feature = "tauri")))]
    fn with_enigo<F>(&self, _ctx: &Context, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut Enigo) -> Result<(), String>,
    {
        let mut enigo = self
            .enigo
            .lock()
            .map_err(|e| format!("Failed to lock the enigo: {}", e))?;

        f(&mut enigo)
    }
}

#[async_trait::async_trait]
impl NodeRunner for KeyboardNodeRunner {
    type ParamType = KeyboardParams;

    async fn run(&mut self, ctx: &Context, params: Self::ParamType) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        match params.mode {
            KeyBoardKeyMode::Type => {
                let value = params
                    .value
                    .ok_or_else(|| "mode Type requires `value`".to_string())?;

                self.with_enigo(ctx, move |enigo| {
                    enigo
                        .text(&value)
                        .map_err(|err| format!("Failed to type text: {err}"))
                })?;
            }
            mode => {
                let key_code = params
                    .key
                    .to_key_code()
                    .ok_or_else(|| format!("Invalid key value '{}'", params.key))?;

                let direction = match mode {
                    KeyBoardKeyMode::Click => Direction::Click,
                    KeyBoardKeyMode::Down => Direction::Press,
                    KeyBoardKeyMode::Up => Direction::Release,
                    KeyBoardKeyMode::Type => unreachable!(),
                };

                self.with_enigo(ctx, move |enigo| {
                    enigo
                        .key(key_code.to_enigo_key(), direction)
                        .map_err(|err| format!("Failed to send key {}: {err}", params.key))
                })?;
            }
        }

        Ok(None)
    }
}

pub struct KeyboardNodeFactory {
    enigo: Arc<Mutex<Enigo>>,
}

impl KeyboardNodeFactory {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Default::default())
            .map_err(|e| e.to_string())
            .unwrap();

        Self {
            enigo: Arc::new(Mutex::new(enigo)),
        }
    }
}

impl NodeRunnerFactory for KeyboardNodeFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(KeyboardNodeRunner::new(self.enigo.clone())))
    }
}
