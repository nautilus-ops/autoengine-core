use crate::context::Context;
use crate::runner::image_recognition::ImageRecognition;
use crate::runner::keyboard::KeyboardRunner;
use crate::runner::mouse::MouseRunner;
use crate::types::{ImageRecognitionParams, KeyBoardKeyMode, KeyBoardParams};
use crate::utils;
use opencv::core::{Mat, Size, Vector};
use opencv::{imgcodecs, imgproc};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "tauri")]
use tauri::AppHandle;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep_until};

mod image_recognition;
pub mod keyboard;
mod mouse;

fn convert_to_int(val: String) -> Option<i64> {
    if val.parse::<i64>().is_ok() {
        Some(val.parse::<i64>().unwrap())
    } else {
        None
    }
}

pub struct ActionRunner {
    mouse: MouseRunner,
    keyboard: KeyboardRunner,
    image_recognition: ImageRecognition,
}

#[cfg(not(feature = "tauri"))]
impl Default for ActionRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRunner {
    #[cfg(feature = "tauri")]
    pub fn new(app: Arc<AppHandle>) -> Self {
        Self {
            mouse: MouseRunner::new(),
            keyboard: KeyboardRunner::new(app),
            image_recognition: ImageRecognition::new(),
        }
    }

    #[cfg(not(feature = "tauri"))]
    pub fn new() -> Self {
        Self {
            mouse: MouseRunner::new(),
            keyboard: KeyboardRunner::new(),
            image_recognition: ImageRecognition::new(),
        }
    }
    pub async fn run_keyboard_action(
        &self,
        _context: &Context,
        _name: &str,
        param: &KeyBoardParams,
        retry: i32,
        interval: u64,
        _duration: Option<u32>,
    ) -> Result<(), String> {
        let key_code = param.key.clone();

        match param.mode {
            KeyBoardKeyMode::Click => {
                handle_retry(retry, interval, 0, || async {
                    self.keyboard
                        .keyboard(key_code.to_enigo_key(), enigo::Direction::Click)
                })
                .await
            }
            KeyBoardKeyMode::Down => {
                handle_retry(retry, interval, 0, || async {
                    self.keyboard
                        .keyboard(key_code.to_enigo_key(), enigo::Direction::Press)
                })
                .await
            }
            KeyBoardKeyMode::Up => {
                handle_retry(retry, interval, 0, || async {
                    self.keyboard
                        .keyboard(key_code.to_enigo_key(), enigo::Direction::Release)
                })
                .await
            }
            KeyBoardKeyMode::Type => {
                let value = param.value.clone().unwrap_or_default();

                handle_retry(retry, interval, 0, || async {
                    self.keyboard.type_values(value.clone())
                })
                .await
            }
        }
    }
    pub async fn run_mouse_move_action(
        &self,
        context: &Context,
        _name: &str,
        x: &str,
        y: &str,
        retry: i32,
    ) -> Result<(), String> {
        let mut variables = utils::parse_variables(context, x).await;
        let x = convert_to_int(variables.clone())
            .ok_or_else(|| format!("Failed to covert x: '{variables}' to int").to_string())?;
        variables = utils::parse_variables(context, y).await;
        let y = convert_to_int(variables.clone())
            .ok_or_else(|| format!("Failed to covert y: '{variables}' to int").to_string())?;

        handle_retry(retry, 500, 0, || async {
            self.mouse
                .run_mouse_move(context.screen_scale, x as i32, y as i32)
        })
        .await
    }

    pub async fn run_mouse_click(
        &self,
        _context: &Context,
        _name: &str,
        value: &str,
        retry: i32,
        interval: u64,
        duration: Option<u32>,
    ) -> Result<(), String> {
        if duration.is_some() {
            handle_retry(retry, interval, 0, || async {
                self.mouse
                    .press_mouse(value, Duration::from_millis(duration.unwrap() as u64))
                    .await
            })
            .await
        } else {
            handle_retry(retry, interval, 0, || async {
                self.mouse.click_mouse(value).await
            })
            .await
        }
    }

    pub async fn run_image_recognition_action(
        &self,
        context: &Context,
        name: &str,
        params: ImageRecognitionParams,
        retry: i32,
        interval: u64,
        result_data: Arc<Mutex<HashMap<&'static str, serde_json::Value>>>,
    ) -> Result<(), String> {
        let recognition = Arc::new(self.image_recognition.clone());

        handle_retry(retry, interval, 200, || async {
            let mut task_set = JoinSet::new();

            // capture and covert to mat
            let pnm_bytes = recognition
                .capture_to_tiff_bytes()
                .map_err(|err| err.to_string())?;
            let screen = Vector::from_slice(&pnm_bytes);
            let mut screen_img = imgcodecs::imdecode(&screen, params.optimization.imread_type())
                .map_err(|err| err.to_string())?;

            log::info!(
                "optimization resize {}, imread_type {:?}",
                params.optimization.resize(),
                params.optimization.imread_type
            );

            if params.optimization.resize() != 1.0 {
                let mut resized = Mat::default();
                imgproc::resize(
                    &screen_img,
                    &mut resized,
                    Size::new(0, 0),
                    params.optimization.resize(),
                    params.optimization.resize(),
                    imgproc::INTER_AREA,
                )
                .map_err(|err| err.to_string())?;
                screen_img = resized;
            }

            for img in &params.images {
                let path = context.pipeline_path.clone();
                let img = img.clone();
                let recognition = recognition.clone();

                let screen_img = screen_img.clone();
                let optimization = params.optimization.clone();
                task_set.spawn(async move {
                    recognition.run_image_recognition(screen_img, &img, path, optimization)
                });
            }

            let mut task_res: Result<(), String> = Ok(());
            while let Some(res) = task_set.join_next().await {
                let mut result_data = result_data.lock().await;

                if let Err(e) = res {
                    task_res = Err(e.to_string());
                    continue;
                }
                if let Ok(result) = res {
                    match result {
                        Ok((point, img, duration, score)) => {
                            log::info!("find image [{img}] point: {:?}", point);

                            // return to ui
                            result_data.insert("image", serde_json::Value::String(img.clone()));
                            result_data.insert(
                                "duration",
                                serde_json::Value::String(format!("{:?}", duration)),
                            );
                            result_data.insert(
                                "score",
                                serde_json::Value::Number(
                                    serde_json::Number::from_f64(score).unwrap(),
                                ),
                            );

                            context
                                .set_string_value(format!("{name}.{img}").as_str(), "true")
                                .await;

                            context
                                .set_string_value(
                                    format!("{name}.{img}.x").as_str(),
                                    format!("{}", point.x).as_str(),
                                )
                                .await;
                            context
                                .set_string_value(
                                    format!("{name}.{img}.y").as_str(),
                                    format!("{}", point.y).as_str(),
                                )
                                .await;

                            log::info!("context update {:?}", context);
                            return Ok(());
                        }
                        Err(e) => {
                            task_res = Err(e.to_string());
                            continue;
                        }
                    }
                }
            }
            task_res
        })
        .await?;
        Ok(())
    }
}

pub async fn handle_retry<F, Fut, R>(
    retry: i32,
    delay_ms: u64,
    min_interval: u64,
    mut handle: F,
) -> Result<R, String>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<R, String>>,
{
    let mut last_err = match handle().await {
        Ok(result) => return Ok(result),
        Err(err) => Some(err),
    };

    let min_interval = Duration::from_millis(min_interval);
    let mut next_tick = Instant::now();

    if retry <= -1 {
        loop {
            match handle().await {
                Ok(res) => return Ok(res),
                Err(err) => {
                    log::error!("{}", err);
                    next_tick += min_interval;

                    let now = Instant::now();
                    if next_tick > now {
                        sleep_until(next_tick).await;
                    } else {
                        next_tick = now;
                    }
                }
            }
        }
    } else {
        for attempt in 0..retry {
            match handle().await {
                Ok(res) => return Ok(res),
                Err(err) => {
                    last_err = Some(err);
                    if attempt + 1 < retry {
                        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }
    }

    Err(last_err.unwrap_or_else(|| "retry failed, unknown error".to_string()))
}
