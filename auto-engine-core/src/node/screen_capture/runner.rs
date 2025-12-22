use crate::context::Context;
use crate::types::node::{NodeRunner, NodeRunnerControl, NodeRunnerController, NodeRunnerFactory};
use screenshots::Screen;
use screenshots::image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::task;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScreenCaptureParams {
    pub mode: String,
    pub file_name: String,
    #[serde(default)]
    pub screen_index: usize,
    #[serde(default)]
    pub x: Option<i32>,
    #[serde(default)]
    pub y: Option<i32>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
}

#[derive(Default, Clone)]
pub struct ScreenCaptureRunner;

impl ScreenCaptureRunner {
    pub fn new() -> Self {
        Self {}
    }

    fn resolve_path(base: &Path, file_name: &str) -> PathBuf {
        let path = PathBuf::from(file_name);
        if path.is_absolute() {
            path
        } else {
            base.join(path)
        }
    }
}

#[async_trait::async_trait]
impl NodeRunner for ScreenCaptureRunner {
    type ParamType = ScreenCaptureParams;

    async fn run(
        &mut self,
        ctx: &Context,
        params: Self::ParamType,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let files_path = ctx.workflow_path.join("files");
        let file_path = Self::resolve_path(&files_path, &params.file_name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create dirs: {}", e))?;
        }

        let mode = params.mode.to_lowercase();
        let screen_index = params.screen_index;
        let capture_task = task::spawn_blocking(move || -> Result<(String, u32, u32), String> {
            let screens = Screen::all().map_err(|e| e.to_string())?;
            let screen = screens.get(screen_index).ok_or_else(|| {
                format!(
                    "Screen index {} is out of range, available: {}",
                    screen_index,
                    screens.len()
                )
            })?;

            let image = if mode == "area" {
                let (x, y, width, height) =
                    match (params.x, params.y, params.width, params.height) {
                        (Some(x), Some(y), Some(width), Some(height)) => (x, y, width, height),
                        _ => {
                            return Err("Area mode requires x, y, width and height to be provided"
                                .to_string());
                        }
                    };
                screen
                    .capture_area(x, y, width, height)
                    .map_err(|e| e.to_string())?
            } else {
                screen.capture().map_err(|e| e.to_string())?
            };

            let width = image.width();
            let height = image.height();

            let dynamic_image = DynamicImage::ImageRgba8(image);
            dynamic_image
                .save(&file_path)
                .map_err(|e| format!("Failed to save screenshot: {}", e))?;

            Ok((params.file_name, width, height))
        });

        let (file_name, width, height) = capture_task
            .await
            .map_err(|e| format!("Failed to capture screen: {}", e))??;

        let mut res = HashMap::new();
        res.insert("file".to_string(), serde_json::json!(file_name));
        res.insert("width".to_string(), serde_json::json!(width));
        res.insert("height".to_string(), serde_json::json!(height));
        Ok(Some(res))
    }
}

pub struct ScreenCaptureRunnerFactory;

impl ScreenCaptureRunnerFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ScreenCaptureRunnerFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeRunnerFactory for ScreenCaptureRunnerFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(ScreenCaptureRunner::new()))
    }
}
