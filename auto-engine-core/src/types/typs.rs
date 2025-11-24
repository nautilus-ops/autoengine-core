use crate::context::Context;
use crate::types::KeyBoardParams;
use crate::types::conditions::Conditions;
use auto_engine_macro::with_metadata;
use opencv::imgcodecs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MouseClickParams {
    pub(crate) value: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MouseMoveParams {
    pub(crate) x: String,
    pub(crate) y: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ImageRecognitionParams {
    pub images: Vec<String>,
    pub sub_pipeline: String,
    #[serde(default)]
    pub optimization: ImageOptimization,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ImageOptimization {
    pub resize: Option<f64>,
    pub imread_type: Option<String>,
}

impl Default for ImageOptimization {
    fn default() -> Self {
        Self {
            resize: Some(1.0),
            imread_type: Some(String::from("GRAYSCALE")),
        }
    }
}

impl ImageOptimization {
    pub fn resize(&self) -> f64 {
        self.resize.unwrap_or(1.0)
    }

    pub fn imread_type(&self) -> i32 {
        let imread_type = self.imread_type.clone().unwrap_or("GRAYSCALE".to_string());
        match imread_type.as_str() {
            "COLOR" => imgcodecs::IMREAD_COLOR,
            "GRAYSCALE" => imgcodecs::IMREAD_GRAYSCALE,
            _ => imgcodecs::IMREAD_GRAYSCALE,
        }
    }
}

pub type Pipeline = Vec<Stage>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Stage {
    pub stage: Vec<Node>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MetaData {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_return: Option<bool>,
}

#[with_metadata]
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "action_type")]
pub enum Node {
    Start,
    KeyBoard { params: KeyBoardParams },
    MouseClick { params: MouseClickParams },
    MouseMove { params: MouseMoveParams },
    ImageRecognition { params: ImageRecognitionParams },
    TimeWait,
    // Customize,
}

impl Node {
    pub fn name(&self) -> &str {
        let metadata = self.metadata();
        &metadata.name
    }

    pub fn conditions(&self) -> Option<Conditions> {
        let metadata = self.metadata();
        metadata.conditions.clone()
    }

    pub async fn check_conditions(&self, ctx: &Context) -> bool {
        let conditions = self.conditions();

        if let Some(conditions) = conditions
            && let Err(err) = conditions.check(ctx).await
        {
            log::debug!("condition check failed, no need to run{}", err);
            return false;
        }
        true
    }

    pub fn stop_when_error(&self) -> bool {
        let metadata = self.metadata();
        metadata.err_return.unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Node, Pipeline};

    #[test]
    fn parse_keyboard_node() {
        let yaml_str = r#"
action_type: KeyBoard
name: "按下W键"
duration: 500
retry: 3
interval: 500
params:
  mode: Down
  key: W
conditions:
  exist: "find-heart.heart.png"
            "#;
        let node: Node = serde_yaml::from_str(yaml_str).unwrap();

        let metadata = node.metadata();

        println!("keyboard node: {:?}", metadata);
    }

    #[test]
    fn parse_pipeline() {
        let yaml_str = r#"
- stage:
    - action_type: Start
      name: "main"
      conditions: {}
- stage:
    - action_type: ImageRecognition
      name: "find-dot"
      retry: -1
      interval: 0
      params:
        images:
          - "dot.png"
        sub_pipeline: ""
      err_return: true
- stage:
    - action_type: MouseMove
      name: "移动鼠标1"
      retry: 2
      params:
        x: "${find-dot.dot.png.x}"
        y: "${find-dot.dot.png.y}"
      conditions:
        exist: "find-dot.dot.png"
- stage:
    - action_type: MouseClick
      name: "点击两个点"
      duration: 100
      retry: 1
      interval: 500
      params:
        value: "left"
      conditions:
        exist: "find-dot.dot.png"
- stage:
    - action_type: ImageRecognition
      name: "find-heart"
      retry: -1
      interval: 0
      params:
        images:
          - "heart.png"
        sub_pipeline: ""
      err_return: true
- stage:
    - action_type: MouseMove
      name: "移动到点赞按钮"
      retry: 2
      params:
        x: "${find-heart.heart.png.x}"
        y: "${find-heart.heart.png.y}"
      conditions:
        exist: "find-heart.heart.png"
- stage:
    - action_type: MouseClick
      name: "点赞"
      duration: 100
      retry: 1
      interval: 500
      params:
        value: "left"
      conditions:
        exist: "find-heart.heart.png"
- stage:
    - action_type: KeyBoard
      name: "按下W键"
      duration: 500
      retry: 3
      interval: 500
      params:
        mode: Down
        key: W
      conditions:
        exist: "find-heart.heart.png"
"#;

        let pipeline: Pipeline = match serde_yaml::from_str(&yaml_str) {
            Ok(res) => res,
            Err(err) => {
                println!("Invalid YAML format {err}");
                assert!(false);
                return;
            }
        };

        println!("success parse pipeline: {:#?}", pipeline);
    }
}
