use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::async_runtime::RwLock;

#[derive(Debug)]
pub struct Context {
    pub string_value: Arc<RwLock<HashMap<String, String>>>,
    pub(crate) screen_scale: f64,
    pub(crate) pipeline_path: PathBuf,
    #[cfg(feature = "tauri")]
    pub(crate) app_handle: Option<tauri::AppHandle>,
}

impl Context {
    
    #[cfg(feature = "tauri")]
    pub fn new(pipeline_path: PathBuf, app_handle: Option<tauri::AppHandle>) -> Self {
        Self {
            string_value: Arc::new(RwLock::new(HashMap::new())),
            screen_scale: 1.0,
            pipeline_path,
            app_handle: app_handle,
        }
    }
    
    #[cfg(not(feature = "tauri"))]
    pub fn new(pipeline_path: PathBuf) -> Self {
        Self {
            string_value: Arc::new(RwLock::new(HashMap::new())),
            screen_scale: 1.0,
            pipeline_path,
        }
    }

    pub fn with_screen_scale(mut self, screen_scale: f64) -> Self {
        self.screen_scale = screen_scale;
        self
    }

    pub(crate) async fn set_string_value(&self, key: &str, value: &str) {
        let mut map = self.string_value.write().await;
        map.insert(key.to_string(), value.to_string());
    }
}
