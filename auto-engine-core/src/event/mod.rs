use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NodeEventPayload {
    pub status: String,
    pub name: String,
    pub result: Option<String>,
}

impl NodeEventPayload {
    pub fn new<D: Serialize>(status: String, name: String, result: Option<D>) -> NodeEventPayload {
        let mut res = None;
        if let Some(data) = result {
            res = Some(serde_json::to_string(&data).unwrap_or_else(|err| format!("{err}")))
        }

        Self {
            status,
            name,
            result: res,
        }
    }

    pub fn running(name: String) -> NodeEventPayload {
        NodeEventPayload::new::<String>("running".to_string(), name, None)
    }

    pub fn skip<D: Serialize>(name: String, result: Option<D>) -> NodeEventPayload {
        NodeEventPayload::new("skip".to_string(), name, result)
    }

    pub fn success<D: Serialize>(name: String, result: Option<D>) -> NodeEventPayload {
        NodeEventPayload::new("done".to_string(), name, result)
    }

    pub fn error<D: Serialize>(name: String, result: Option<D>) -> NodeEventPayload {
        NodeEventPayload::new("error".to_string(), name, result)
    }

    pub fn cancel() -> NodeEventPayload {
        NodeEventPayload::new::<String>("cancel".to_string(), "*".to_string(), None)
    }
}
