use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::async_runtime::RwLock;

#[derive(Debug)]
pub struct Context {
    pub string_value: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    pub(crate) screen_scale: f64,
    pub(crate) pipeline_path: PathBuf,
    pub(crate) workflow_path: PathBuf,
    #[cfg(feature = "tauri")]
    pub(crate) app_handle: Option<tauri::AppHandle>,
}

impl Context {
    #[cfg(feature = "tauri")]
    pub fn new(path: PathBuf, app_handle: Option<tauri::AppHandle>) -> Self {
        Self {
            string_value: Arc::new(RwLock::new(HashMap::new())),
            screen_scale: 1.0,
            pipeline_path: path.clone(),
            workflow_path: path.clone(),
            app_handle,
        }
    }

    #[cfg(not(feature = "tauri"))]
    pub fn new(path: PathBuf) -> Self {
        Self {
            string_value: Arc::new(RwLock::new(HashMap::new())),
            screen_scale: 1.0,
            pipeline_path: path.clone(),
            workflow_path: path.clone(),
        }
    }

    pub fn with_screen_scale(mut self, screen_scale: f64) -> Self {
        self.screen_scale = screen_scale;
        self
    }

    pub async fn set_string_value(&self, key: &str, value: &str) -> Result<(), String> {
        self.set_value::<String>(key, value.to_string()).await
    }

    pub async fn set_value<T: Serialize>(&self, key: &str, value: T) -> Result<(), String> {
        let mut map = self.string_value.write().await;
        map.insert(
            key.to_string(),
            serde_json::to_value(value).map_err(|e| format!("{:?}", e))?,
        );
        Ok(())
    }

    pub fn load_image_path(&self, image: &str) -> Result<PathBuf, String> {
        let image_path = self.workflow_path.join("images").join(image);
        if !image_path.exists() {
            return Err(format!("Image {} does not exist", image));
        }
        Ok(image_path)
    }
}
