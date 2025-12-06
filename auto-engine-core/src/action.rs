pub mod screenshot;

pub struct Action {}

impl Action {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle<T>(action: String) -> Result<T, String> {
        match action.as_str() {
            "screenshot" => {}
            _ => {}
        }

        Err(String::from("Action handle not implemented yet"))
    }
}
