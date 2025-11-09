use crate::context::Context;
use crate::types::KeyBoardParams;
use crate::types::conditions::Conditions;
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

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "action_type")]
pub enum Node {
    Start {
        name: String,
        conditions: String,
    },
    KeyBoard {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        retry: i32,
        interval: u64,
        params: KeyBoardParams,
        #[serde(skip_serializing_if = "Option::is_none")]
        conditions: Option<Conditions>,
        #[serde(skip_serializing_if = "Option::is_none")]
        err_return: Option<bool>,
    },
    MouseClick {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        retry: i32,
        interval: u64,
        params: MouseClickParams,
        #[serde(skip_serializing_if = "Option::is_none")]
        conditions: Option<Conditions>,
        #[serde(skip_serializing_if = "Option::is_none")]
        err_return: Option<bool>,
    },
    MouseMove {
        name: String,
        retry: i32,
        params: MouseMoveParams,
        #[serde(skip_serializing_if = "Option::is_none")]
        conditions: Option<Conditions>,
        #[serde(skip_serializing_if = "Option::is_none")]
        err_return: Option<bool>,
    },
    ImageRecognition {
        name: String,
        params: ImageRecognitionParams,
        retry: i32,
        interval: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        conditions: Option<Conditions>,
        #[serde(skip_serializing_if = "Option::is_none")]
        err_return: Option<bool>,
    },
    TimeWait {
        name: String,
        duration: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        conditions: Option<Conditions>,
    },
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Start { name, .. } => name,
            Node::KeyBoard { name, .. } => name,
            Node::MouseClick { name, .. } => name,
            Node::MouseMove { name, .. } => name,
            Node::ImageRecognition { name, .. } => name,
            Node::TimeWait { name, .. } => name,
        }
    }

    pub fn conditions(&self) -> Option<Conditions> {
        match self {
            Node::Start { .. } => None,
            Node::KeyBoard { conditions, .. } => conditions.clone(),
            Node::MouseClick { conditions, .. } => conditions.clone(),
            Node::MouseMove { conditions, .. } => conditions.clone(),
            Node::ImageRecognition { conditions, .. } => conditions.clone(),
            Node::TimeWait { conditions, .. } => conditions.clone(),
        }
    }

    pub async fn check_conditions(&self, ctx: &Context) -> bool {
        let conditions = match self {
            Node::Start { .. } => return true,
            Node::KeyBoard { conditions, .. } => conditions,
            Node::MouseClick { conditions, .. } => conditions,
            Node::MouseMove { conditions, .. } => conditions,
            Node::ImageRecognition { conditions, .. } => conditions,
            Node::TimeWait { conditions, .. } => conditions,
        };
        if let Some(conditions) = conditions
            && let Err(err) = conditions.check(ctx).await
        {
            log::debug!("condition check failed, no need to run{}", err);
            return false;
        }
        true
    }

    pub fn stop_when_error(&self) -> bool {
        match self {
            Node::Start { .. } => true,
            Node::KeyBoard { err_return, .. } => err_return.unwrap_or(true),
            Node::MouseClick { err_return, .. } => err_return.unwrap_or(true),
            Node::MouseMove { err_return, .. } => err_return.unwrap_or(true),
            Node::ImageRecognition { err_return, .. } => err_return.unwrap_or(true),
            Node::TimeWait { .. } => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Pipeline;

    #[test]
    fn parse_pipeline() {
        let yaml_str = r#"
- stage:
    - action_type: Start
      name: "main"
      conditions: ""
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
        key: "w"
      conditions:
        exist: "find-heart.heart.png"
"#;

        let pipeline: Pipeline = match serde_yaml::from_str(&yaml_str) {
            Ok(res) => res,
            _ => {
                println!("Invalid YAML format");
                assert!(false);
                return;
            }
        };

        println!("success parse pipeline: {:#?}", pipeline);
    }
}
