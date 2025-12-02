use serde_json::Value;

pub trait Emitter {
    fn emit(&self, event: &str, payload: Value) -> Result<(), String>;
}

pub struct NotificationEmitter {
    emitters: Vec<Box<dyn Emitter>>,
}

impl NotificationEmitter {
    pub fn new() -> Self {
        Self { emitters: vec![] }
    }

    pub fn with_emitter(mut self, emitter: Box<dyn Emitter>) -> Self {
        self.emitters.push(emitter);
        self
    }

    pub fn emit<S: serde::Serialize + Clone>(&self, event: &str, payload: S) -> Result<(), String> {
        for emitter in self.emitters.iter() {
            let payload = serde_json::to_value(&payload).map_err(|e| e.to_string())?;
            emitter.emit(event, payload)?
        }
        Ok(())
    }
}
