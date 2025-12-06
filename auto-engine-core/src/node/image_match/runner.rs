use crate::action;
use crate::context::Context;
use crate::types::node::{NodeRunner, NodeRunnerControl, NodeRunnerController, NodeRunnerFactory};
use opencv::core::{Mat, MatTraitConst, Point, Size};
use opencv::imgproc::TM_CCOEFF_NORMED;
use opencv::{imgcodecs, imgproc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ImageMatchParams {
    target_score: f32,
    template_image: String,
    source_image: String,
    imread_type: String,
    use_screenshot: bool,
    resize: f64,
}

pub struct ImageMatchRunner {
    template_image: Option<Mat>,
}

impl ImageMatchRunner {
    pub fn new() -> Self {
        Self {
            template_image: None,
        }
    }

    fn resize_mat(&self, mat: &Mat, resize: f64) -> Result<Mat, String> {
        let mut mat = mat.clone();
        if resize != 1.0 {
            let mut resized = Mat::default();
            imgproc::resize(
                &mat,
                &mut resized,
                Size::new(0, 0),
                resize,
                resize,
                imgproc::INTER_AREA,
            )
            .map_err(|err| err.to_string())?;
            mat = resized;
        }
        Ok(mat)
    }
}

impl Default for ImageMatchRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl NodeRunner for ImageMatchRunner {
    type ParamType = ImageMatchParams;

    async fn run(
        &mut self,
        ctx: &Context,
        param: Self::ParamType,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let imread_mode = match param.imread_type.to_uppercase().as_str() {
            "GRAYSCALE" => imgcodecs::IMREAD_GRAYSCALE,
            "COLOR" => imgcodecs::IMREAD_COLOR,
            _ => imgcodecs::IMREAD_GRAYSCALE,
        };
        let files_folder = ctx.workflow_path.join("files");

        let mut template_mat = if let Some(mat) = &self.template_image {
            // load template from cache
            mat.clone()
        } else {
            let mat = imgcodecs::imread(
                files_folder
                    .join(&param.template_image)
                    .to_string_lossy()
                    .as_ref(),
                imread_mode,
            )
            .map_err(|e| e.to_string())?;
            self.template_image = Some(mat.clone());
            mat
        };

        log::info!("resize ===> {}", param.resize);

        let start = Instant::now();

        let mut source_mat = if param.use_screenshot {
            action::screenshot::capture_to_mat(imread_mode).map_err(|e| e.to_string())?
        } else {
            imgcodecs::imread(
                files_folder
                    .join(&param.source_image)
                    .to_string_lossy()
                    .as_ref(),
                imread_mode,
            )
            .map_err(|e| e.to_string())?
        };

        let duration = start.elapsed();
        log::info!(
            "capture_screen took {:?}, path {}",
            duration,
            param.template_image
        );


        template_mat = self.resize_mat(&template_mat, param.resize)?;
        source_mat = self.resize_mat(&source_mat, param.resize)?;

        let mut result = Mat::default();
        imgproc::match_template(
            &source_mat,
            &template_mat,
            &mut result,
            TM_CCOEFF_NORMED,
            &Mat::default(),
        )
        .map_err(|e| e.to_string())?;

        let mut max_val = 0.0;
        let mut max_loc = Point::default();

        opencv::core::min_max_loc(
            &result,
            None,
            Some(&mut max_val),
            None,
            Some(&mut max_loc),
            &Mat::default(),
        )
        .map_err(|e| e.to_string())?;

        log::info!("match score: {:.3}", max_val);

        let (x, y, score) = if max_val > 0.8 {
            let template_size = template_mat.size().map_err(|e| e.to_string())?;
            let center_x = max_loc.x + template_size.width / 2;
            let center_y = max_loc.y + template_size.height / 2;

            log::info!("center x: {}, center y: {}", center_x, center_y);
            log::info!(
                "resized center x: {},resized center y: {}",
                center_x,
                center_y
            );
            (center_x, center_y, max_val)
        } else {
            return Err(
                format!("Failed to find image image. max image score {max_val}")
                    .to_string()
                    .into(),
            );
        };

        let mut res = HashMap::new();
        res.insert(
            format!("{}.score", param.template_image),
            serde_json::json!(score),
        );
        res.insert(format!("{}.x", param.template_image), serde_json::json!(x));
        res.insert(format!("{}.y", param.template_image), serde_json::json!(y));
        res.insert(
            format!("{}.cost_time", param.template_image),
            serde_json::json!(duration.as_secs_f32()),
        );

        Ok(Some(res))
    }
}

#[derive(Default)]
pub struct ImageMatchRunnerFactory;

impl ImageMatchRunnerFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeRunnerFactory for ImageMatchRunnerFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(ImageMatchRunner::new()))
    }
}
