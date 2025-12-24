use crate::context::Context;
use crate::types::node::{NodeRunner, NodeRunnerControl, NodeRunnerController, NodeRunnerFactory};
use leptess::{LepTess, Variable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::task;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OcrParams {
    pub image: String,
    pub language: String,
    pub whitelist: String,
    pub numeric_mode: bool,
    pub digits_only: bool,
}

#[derive(Default, Clone)]
pub struct OcrRunner;

impl OcrRunner {
    pub fn new() -> Self {
        Self {}
    }

    fn resolve_path(base: &Path, image: &str) -> PathBuf {
        let path = PathBuf::from(image);
        if path.is_absolute() {
            path
        } else {
            base.join(path)
        }
    }
}

#[async_trait::async_trait]
impl NodeRunner for OcrRunner {
    type ParamType = OcrParams;

    async fn run(
        &mut self,
        ctx: &Context,
        param: Self::ParamType,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let image_path = Self::resolve_path(&ctx.workflow_path.join("files"), &param.image);
        if !image_path.exists() {
            return Err(format!(
                "Image '{}' does not exist",
                image_path.to_string_lossy()
            ));
        }

        let task_param = param.clone();
        let text = task::spawn_blocking(move || -> Result<String, String> {
            let mut lt =
                LepTess::new(None, &task_param.language).map_err(|e| format!("{:?}", e))?;

            if !task_param.whitelist.is_empty() {
                lt.set_variable(Variable::TesseditCharWhitelist, &task_param.whitelist)
                    .map_err(|e| format!("{:?}", e))?;
            }

            if task_param.numeric_mode {
                lt.set_variable(Variable::ClassifyBlnNumericMode, "1")
                    .map_err(|e| format!("{:?}", e))?;
            }

            lt.set_image(image_path.to_string_lossy().as_ref())
                .map_err(|e| format!("{:?}", e))?;
            let text = lt.get_utf8_text().map_err(|e| format!("{:?}", e))?;

            let cleaned = if task_param.digits_only {
                text.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
            } else {
                text.trim().to_string()
            };

            Ok(cleaned)
        })
        .await
        .map_err(|e| format!("Failed to run OCR: {}", e))??;

        let mut res = HashMap::new();
        res.insert("text".to_string(), serde_json::json!(text));
        Ok(Some(res))
    }
}

pub struct OcrRunnerFactory;

impl OcrRunnerFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OcrRunnerFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeRunnerFactory for OcrRunnerFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(OcrRunner::new()))
    }
}
