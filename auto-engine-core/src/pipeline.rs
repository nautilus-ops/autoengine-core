use crate::context::Context;
use crate::runner::ActionRunner;
use crate::types::{Node, Pipeline, Stage};
#[cfg(feature = "tauri")]
use crate::{event, event::TaskStatus};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "tauri")]
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time;
use tokio::time::{Instant, sleep, sleep_until};
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
enum RunningResult {
    Pass,
    Error(String),
}

pub struct PipelineRunner {
    pipelines: Vec<Pipeline>,
    pub rate: f64,
    pub dir_path: PathBuf,
    pub min_interval_ms: u64,
}

impl PipelineRunner {
    pub fn new(
        pipelines: Vec<Pipeline>,
        rate: f64,
        dir_path: PathBuf,
        min_interval_ms: u64,
    ) -> Self {
        PipelineRunner {
            pipelines,
            rate,
            dir_path,
            min_interval_ms,
        }
    }
}

#[cfg(feature = "tauri")]
impl PipelineRunner {
    pub fn run(
        &self,
        token: &std::sync::Mutex<Arc<CancellationToken>>,
        is_loop: bool,
        app: AppHandle,
    ) -> Result<(), String> {
        handle_pipeline(
            token,
            self.dir_path.clone(),
            &self.pipelines,
            is_loop,
            self.rate,
            self.min_interval_ms,
            app,
        )
    }
}

#[cfg(not(feature = "tauri"))]
impl PipelineRunner {
    pub fn run(
        &self,
        token: &std::sync::Mutex<Arc<CancellationToken>>,
        is_loop: bool,
    ) -> Result<(), String> {
        handle_pipeline(
            token,
            self.dir_path.clone(),
            &self.pipelines,
            is_loop,
            self.rate,
            self.min_interval_ms,
        )
    }
}

#[cfg(feature = "tauri")]
fn handle_pipeline(
    token: &std::sync::Mutex<Arc<CancellationToken>>,
    dir_path: PathBuf,
    pipelines: &Vec<Pipeline>,
    is_loop: bool,
    rate: f64,
    min_interval_ms: u64,
    app: AppHandle,
) -> Result<(), String> {
    let app = Arc::new(app);
    let mut task_set = JoinSet::new();

    for pipeline in pipelines {
        let pipeline = pipeline.clone();
        let dir_path = dir_path.clone();
        let runner = Arc::new(ActionRunner::new(app.clone()));
        log::info!("Running pipeline {pipeline:?}");

        let app_handle = app.clone();

        let handle = async move {
            let min_interval = Duration::from_millis(min_interval_ms.max(10));
            let mut next_tick = Instant::now();

            loop {
                app_handle
                    .emit(
                        event::TASK_EVENT,
                        event::TaskEventPayload {
                            status: TaskStatus::Running,
                        },
                    )
                    .unwrap();

                let pipeline = pipeline.clone();
                let dir_path = dir_path.clone();
                let context = Arc::new(Context::new(dir_path).with_screen_scale(rate));

                for stage in pipeline {
                    match handle_stage_with_app(
                        context.clone(),
                        stage,
                        runner.clone(),
                        app_handle.clone(),
                    )
                    .await
                    {
                        RunningResult::Pass => {
                            sleep(Duration::from_millis(100)).await;
                        }
                        RunningResult::Error(err) => {
                            log::error!("Terminated execution, pipeline error: {err}");
                            return;
                        }
                    };
                }

                app_handle
                    .emit(
                        event::TASK_EVENT,
                        event::TaskEventPayload {
                            status: TaskStatus::Finished,
                        },
                    )
                    .unwrap();

                if !is_loop {
                    return;
                }
                next_tick += min_interval;

                let now = Instant::now();
                if next_tick > now {
                    sleep_until(next_tick).await;
                } else {
                    next_tick = now;
                }
            }
        };

        let token = token.lock().unwrap().clone();

        task_set.spawn(async move {
            tokio::select! {
                _ = token.cancelled() => {
                    log::info!("Pipeline terminated, exiting loop");
                }
                _ = handle => {}
            }
        });
    }

    let app_handle = app.clone();

    tokio::task::spawn(async move {
        while task_set.join_next().await.is_some() {}
        app_handle.emit("pipeline", "stopping").unwrap_or_default();
        app_handle
            .emit("node", event::NodeEventPayload::cancel())
            .unwrap_or_default();
    });

    Ok(())
}

#[cfg(not(feature = "tauri"))]
fn handle_pipeline(
    token: &std::sync::Mutex<Arc<CancellationToken>>,
    dir_path: PathBuf,
    pipelines: &Vec<Pipeline>,
    is_loop: bool,
    rate: f64,
    min_interval_ms: u64,
) -> Result<(), String> {
    let mut task_set = JoinSet::new();

    for pipeline in pipelines {
        let pipeline = pipeline.clone();
        let dir_path = dir_path.clone();
        let runner = Arc::new(ActionRunner::new());
        log::info!("Running pipeline {pipeline:?}");

        let handle = async move {
            let min_interval = Duration::from_millis(min_interval_ms.max(10));
            let mut next_tick = Instant::now();

            loop {
                let pipeline = pipeline.clone();
                let dir_path = dir_path.clone();
                let context = Arc::new(Context::new(dir_path).with_screen_scale(rate));

                for stage in pipeline {
                    match handle_stage_without_app(context.clone(), stage, runner.clone()).await {
                        RunningResult::Pass => {
                            sleep(Duration::from_millis(100)).await;
                        }
                        RunningResult::Error(err) => {
                            log::error!("Terminated execution, pipeline error: {err}");
                            return;
                        }
                    };
                }

                if !is_loop {
                    return;
                }
                next_tick += min_interval;

                let now = Instant::now();
                if next_tick > now {
                    sleep_until(next_tick).await;
                } else {
                    next_tick = now;
                }
            }
        };

        let token = token.lock().unwrap().clone();

        task_set.spawn(async move {
            tokio::select! {
                _ = token.cancelled() => {
                    log::info!("Pipeline terminated, exiting loop");
                }
                _ = handle => {}
            }
        });
    }

    tokio::task::spawn(async move { while task_set.join_next().await.is_some() {} });

    Ok(())
}

#[cfg(feature = "tauri")]
async fn handle_stage_with_app(
    context: Arc<Context>,
    stage: Stage,
    runner: Arc<ActionRunner>,
    app: Arc<AppHandle>,
) -> RunningResult {
    let mut task_set = JoinSet::new();
    for node in stage.stage {
        let runner = runner.clone();
        let context = context.clone();
        let app = app.clone();

        task_set.spawn(async move { handle_node_with_app(context, node, runner, app).await });
    }

    let mut stage_result = RunningResult::Pass;

    while let Some(res) = task_set.join_next().await {
        if let Err(e) = res {
            stage_result = RunningResult::Error(e.to_string());
            break;
        }
        if let Ok(result) = res {
            stage_result = result;
        }
    }

    stage_result
}

#[cfg(not(feature = "tauri"))]
async fn handle_stage_without_app(
    context: Arc<Context>,
    stage: Stage,
    runner: Arc<ActionRunner>,
) -> RunningResult {
    let mut task_set = JoinSet::new();
    for node in stage.stage {
        let runner = runner.clone();
        let context = context.clone();

        task_set.spawn(async move { handle_node_without_app(context, node, runner).await });
    }

    let mut stage_result = RunningResult::Pass;

    while let Some(res) = task_set.join_next().await {
        if let Err(e) = res {
            stage_result = RunningResult::Error(e.to_string());
            break;
        }
        if let Ok(result) = res {
            stage_result = result;
        }
    }

    stage_result
}

#[cfg(feature = "tauri")]
async fn handle_node_with_app(
    context: Arc<Context>,
    node: Node,
    runner: Arc<ActionRunner>,
    app: Arc<AppHandle>,
) -> RunningResult {
    let node_name = node.name().to_string();
    let result_data: Arc<Mutex<HashMap<&'static str, serde_json::Value>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let error_return = node.stop_when_error();
    let conditions =
        serde_json::to_value(node.conditions().unwrap_or_default()).unwrap_or_default();

    if !node.check_conditions(&context).await {
        app.emit(
            "node",
            event::NodeEventPayload::skip(node_name.clone(), Some(conditions)),
        )
        .unwrap_or_default();
        return RunningResult::Pass;
    }

    app.emit("node", event::NodeEventPayload::running(node_name.clone()))
        .unwrap_or_default();

    let res: Result<(), String> = match node {
        Node::Start { .. } => Ok(()),
        Node::KeyBoard {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();
            let interval = metadata.interval.unwrap_or_default();
            log::info!("Running pipeline keyboard: {:?}", &params);
            runner
                .run_keyboard_action(
                    &context,
                    &metadata.name,
                    &params,
                    retry,
                    interval,
                    metadata.duration,
                )
                .await
        }
        Node::MouseClick {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();
            let interval = metadata.interval.unwrap_or_default();

            log::info!("Mouse click {}", params.value);
            runner
                .run_mouse_click(
                    &context,
                    &metadata.name,
                    &params.value,
                    retry,
                    interval,
                    metadata.duration,
                )
                .await
        }
        Node::MouseMove {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();

            log::info!("{:?} {:?}", metadata.name, params);
            runner
                .run_mouse_move_action(&context, &metadata.name, &params.x, &params.y, retry)
                .await
        }
        Node::ImageRecognition {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();
            let interval = metadata.interval.unwrap_or_default();

            log::info!("ImageRecognition for image: {:?}", params.images);
            runner
                .run_image_recognition_action(
                    &context,
                    &metadata.name,
                    params,
                    retry,
                    interval,
                    result_data.clone(),
                )
                .await
        }
        Node::TimeWait { metadata, .. } => {
            let duration = metadata.duration.unwrap_or_default() as u64;
            let name = metadata.name;

            log::info!("Running pipeline time wait {duration}, name {name}");
            time::sleep(Duration::from_millis(duration)).await;
            Ok(())
        }
    };

    match res {
        Ok(_) => {
            log::info!("node successfully executed {node_name}");
            let mut result_data = result_data.lock().await;
            result_data.insert("conditions", conditions);

            app.emit(
                "node",
                event::NodeEventPayload::success(node_name, Some(result_data.clone())),
            )
            .unwrap_or_default();
            RunningResult::Pass
        }
        Err(err) => {
            log::error!("handle {err}");
            let mut result_data = result_data.lock().await;
            result_data.insert("error", serde_json::Value::String(err.to_string()));
            app.emit(
                "node",
                event::NodeEventPayload::error(node_name, Some(result_data.clone())),
            )
            .unwrap_or_default();
            if !error_return {
                return RunningResult::Pass;
            }

            RunningResult::Error(err)
        }
    }
}

#[cfg(not(feature = "tauri"))]
async fn handle_node_without_app(
    context: Arc<Context>,
    node: Node,
    runner: Arc<ActionRunner>,
) -> RunningResult {
    let node_name = node.name().to_string();
    let result_data: Arc<Mutex<HashMap<&'static str, serde_json::Value>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let error_return = node.stop_when_error();
    let conditions =
        serde_json::to_value(node.conditions().unwrap_or_default()).unwrap_or_default();

    if !node.check_conditions(&context).await {
        return RunningResult::Pass;
    }

    let res: Result<(), String> = match node {
        Node::Start { .. } => Ok(()),
        Node::KeyBoard {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();
            let interval = metadata.interval.unwrap_or_default();
            log::info!("Running pipeline keyboard: {:?}", &params);
            runner
                .run_keyboard_action(
                    &context,
                    &metadata.name,
                    &params,
                    retry,
                    interval,
                    metadata.duration,
                )
                .await
        }
        Node::MouseClick {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();
            let interval = metadata.interval.unwrap_or_default();

            log::info!("Mouse click {}", params.value);
            runner
                .run_mouse_click(
                    &context,
                    &metadata.name,
                    &params.value,
                    retry,
                    interval,
                    metadata.duration,
                )
                .await
        }
        Node::MouseMove {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();

            log::info!("{:?} {:?}", metadata.name, params);
            runner
                .run_mouse_move_action(&context, &metadata.name, &params.x, &params.y, retry)
                .await
        }
        Node::ImageRecognition {
            metadata, params, ..
        } => {
            let retry = metadata.retry.unwrap_or_default();
            let interval = metadata.interval.unwrap_or_default();

            log::info!("ImageRecognition for image: {:?}", params.images);
            runner
                .run_image_recognition_action(
                    &context,
                    &metadata.name,
                    params,
                    retry,
                    interval,
                    result_data.clone(),
                )
                .await
        }
        Node::TimeWait { metadata, .. } => {
            let duration = metadata.duration.unwrap_or_default() as u64;
            let name = metadata.name;

            log::info!("Running pipeline time wait {duration}, name {name}");
            time::sleep(Duration::from_millis(duration)).await;
            Ok(())
        }
    };

    match res {
        Ok(_) => {
            log::info!("node successfully executed {node_name}");
            let mut result_data = result_data.lock().await;
            result_data.insert("conditions", conditions);
            RunningResult::Pass
        }
        Err(err) => {
            log::error!("handle {err}");
            let mut result_data = result_data.lock().await;
            result_data.insert("error", serde_json::Value::String(err.to_string()));
            if !error_return {
                return RunningResult::Pass;
            }

            RunningResult::Error(err)
        }
    }
}
