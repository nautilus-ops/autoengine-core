use enigo::Direction::{Click, Press, Release};
use enigo::{Button, Coordinate, Enigo, Mouse};
use std::time::Duration;
use tokio::time;

#[derive(Default)]
pub struct MouseRunner {}
impl MouseRunner {
    pub fn new() -> MouseRunner {
        MouseRunner {}
    }

    pub fn run_mouse_move(&self, rate: f64, x: i32, y: i32) -> Result<(), String> {
        let mut enigo = Enigo::new(&Default::default()).map_err(|e| e.to_string())?;

        enigo
            .move_mouse(x / rate as i32, y / rate as i32, Coordinate::Abs)
            .map_err(|err| format!("Failed to move_mouse: {err}"))?;
        Ok(())
    }

    pub async fn click_mouse(&self, value: &str) -> Result<(), String> {
        let mut enigo = Enigo::new(&Default::default()).map_err(|e| e.to_string())?;

        let btn = match value.to_lowercase().as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            _ => {
                return Err(format!("Invalid button value '{}'", value));
            }
        };

        enigo
            .button(btn, Click)
            .map_err(|err| format!("Failed to click {value}: {err}"))?;
        Ok(())
    }

    pub async fn press_mouse(&self, value: &str, duration: Duration) -> Result<(), String> {
        let mut enigo = Enigo::new(&Default::default()).map_err(|e| e.to_string())?;

        let btn = match value.to_lowercase().as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            _ => {
                return Err(format!("Invalid button value '{}'", value));
            }
        };

        enigo
            .button(btn, Press)
            .map_err(|err| format!("Failed to press mouse {value}: {err}"))?;

        time::sleep(duration).await;

        enigo
            .button(btn, Release)
            .map_err(|err| format!("Failed to release mouse {value}: {err}"))?;

        Ok(())
    }
}
