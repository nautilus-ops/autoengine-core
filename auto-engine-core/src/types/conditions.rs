use crate::context::Context;
use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Conditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_exist: Option<String>,
}

impl Conditions {
    pub async fn check(&self, ctx: &Context) -> Result<bool, String> {
        if let Some(key) = &self.exist {
            let values = ctx.string_value.read().await;
            if !values.contains_key(key) {
                log::info!("{} does not exist, {:?}", key, values);
                return Ok(false);
            }
        }

        if let Some(key) = &self.not_exist {
            let values = ctx.string_value.read().await;
            if values.contains_key(key) {
                log::info!("{} does not exist, {:?}", key, values);
                return Ok(false);
            }
        }

        if let Some(condition) = &self.condition {
            let condition = utils::parse_variables(ctx, condition).await;
            let result = evalexpr::eval_boolean(&condition).map_err(|err| format!("{} is not boolean", err))?;
            if !result {
                log::info!("{} does not pass condition", condition);
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    #[tokio::test]
    async fn test_conditions() {
        struct TestCase {
            name: &'static str,
            conditions: Conditions,
            want_err: bool,
        }

        let tests: &[TestCase] = &[
            TestCase {
                name: "x exists and passes condition",
                conditions: Conditions {
                    exist: s("image.x"),
                    condition: s("${image.x} > 1"),
                    not_exist: None,
                },
                want_err: false,
            },
            TestCase {
                name: "y does not exist, should fail",
                conditions: Conditions {
                    exist: s("image.y"),
                    condition: s("${image.y} > 1"),
                    not_exist: None,
                },
                want_err: true,
            },
            TestCase {
                name: "y exist, should fail",
                conditions: Conditions {
                    exist: None,
                    condition: s("${image.y} > 1"),
                    not_exist: s("image.y"),
                },
                want_err: true,
            },
        ];

        #[cfg(feature = "tauri")]
        let context = Context::new(PathBuf::new(), None);

        #[cfg(not(feature = "tauri"))]
        let context = Context::new(PathBuf::new());

        context.set_string_value("image.x", "2").await.unwrap();

        for t in tests {
            let got_err = t.conditions.check(&context).await.is_err();
            assert_eq!(
                got_err, t.want_err,
                "test `{}` failed: conditions={:?}",
                t.name, t.conditions
            );
        }
    }
}
