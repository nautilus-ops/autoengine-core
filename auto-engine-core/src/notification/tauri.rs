use tauri::{AppHandle, Emitter};

use crate::notification::emitter::Emitter as AutoEngineEmitter;

pub struct TauriEmitter {
    app_handle: AppHandle,
}

impl TauriEmitter {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
}

impl AutoEngineEmitter for TauriEmitter {
    fn emit(&self, event: &str, payload: serde_json::Value) -> Result<(), String> {
        self.app_handle
            .emit(event, payload)
            .map_err(|e| e.to_string())
    }
}
