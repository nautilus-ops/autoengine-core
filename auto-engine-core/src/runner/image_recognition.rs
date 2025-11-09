use crate::types::ImageOptimization;
use opencv::core::Size;
use opencv::{
    core::{self, Mat, Point},
    imgcodecs::{self},
    imgproc::{self, TM_CCOEFF_NORMED},
    prelude::*,
};
use screenshots::Screen;
use screenshots::image::{DynamicImage, ImageFormat};
use std::io::Cursor;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct ImageRecognition {}

impl ImageRecognition {
    pub fn new() -> ImageRecognition {
        ImageRecognition {}
    }

    fn find_icon(
        &self,
        screen_img: Mat,
        template_path: &str,
        optimization: ImageOptimization,
    ) -> Result<(Point, f64), Box<dyn std::error::Error>> {
        let mut template = imgcodecs::imread(template_path, optimization.imread_type())?;

        if optimization.resize() != 1.0 {
            let mut resized = Mat::default();
            imgproc::resize(
                &template,
                &mut resized,
                Size::new(0, 0),
                optimization.resize(),
                optimization.resize(),
                imgproc::INTER_AREA,
            )
            .map_err(|err| err.to_string())?;
            template = resized;
        }

        let mut result = Mat::default();
        imgproc::match_template(
            &screen_img,
            &template,
            &mut result,
            TM_CCOEFF_NORMED,
            &Mat::default(),
        )?;

        let mut max_val = 0.0;
        let mut max_loc = Point::default();

        core::min_max_loc(
            &result,
            None,
            Some(&mut max_val),
            None,
            Some(&mut max_loc),
            &Mat::default(),
        )?;

        log::info!("match score: {:.3}", max_val);

        if max_val > 0.8 {
            let template_size = template.size()?;
            let center_x = max_loc.x + template_size.width / 2;
            let center_y = max_loc.y + template_size.height / 2;

            log::info!("center x: {}, center y: {}", center_x, center_y);
            log::info!(
                "resized center x: {},resized center y: {}",
                (center_x as f64 / optimization.resize()) as i32,
                (center_y as f64 / optimization.resize()) as i32
            );
            Ok((
                Point::new(
                    (center_x as f64 / optimization.resize()) as i32,
                    (center_y as f64 / optimization.resize()) as i32,
                ),
                max_val,
            ))
        } else {
            Err(
                format!("Failed to find image image. max image score {max_val}")
                    .to_string()
                    .into(),
            )
        }
    }

    pub fn run_image_recognition(
        &self,
        source: Mat,
        img: &str,
        path: PathBuf,
        optimization: ImageOptimization,
    ) -> Result<(Point, String, Duration, f64), String> {
        let start = Instant::now();

        let image_path = path.join("image").join(img).display().to_string();

        let result = self
            .find_icon(source, &image_path, optimization)
            .map_err(|err| err.to_string());

        let duration = start.elapsed();
        log::info!("capture_screen took {:?}, path {img}", duration);
        let (point, score) = result?;
        Ok((point, img.to_string(), duration, score))
    }

    pub fn capture_to_tiff_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let screen = Screen::all()?.into_iter().next().unwrap();
        let image = screen.capture()?;

        let dynamic_image = DynamicImage::ImageRgba8(image);

        let mut bytes: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);

        dynamic_image.write_to(&mut cursor, ImageFormat::Tiff)?;
        Ok(bytes)
    }
}
