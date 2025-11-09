use enigo::Direction::{Press, Release};
use enigo::{Enigo, Key, Keyboard};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
#[cfg(feature = "tauri")]
use tauri::AppHandle;

pub struct KeyboardRunner {
    #[cfg(feature = "tauri")]
    app: Arc<AppHandle>,
    enigo: Arc<Mutex<Enigo>>,
}

impl KeyboardRunner {
    #[cfg(feature = "tauri")]
    pub fn new(app: Arc<AppHandle>) -> Self {
        let enigo = Enigo::new(&Default::default())
            .map_err(|e| format!("Failed to init Enigo: {e}"))
            .unwrap();

        KeyboardRunner {
            #[cfg(feature = "tauri")]
            app,
            enigo: Arc::new(Mutex::new(enigo)),
        }
    }

    #[cfg(not(feature = "tauri"))]
    pub fn new() -> Self {
        let enigo = Enigo::new(&Default::default())
            .map_err(|e| format!("Failed to init Enigo: {e}"))
            .unwrap();

        KeyboardRunner {
            enigo: Arc::new(Mutex::new(enigo)),
        }
    }

    #[cfg(all(target_os = "macos", feature = "tauri"))]
    pub fn keyboard(&self, key: Key, direction: enigo::Direction) -> Result<(), String> {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel::<Result<(), String>>();

        let lock = self.enigo.clone();

        let result = self.app.run_on_main_thread(move || {
            let result = (|| {
                let mut enigo = lock
                    .lock()
                    .map_err(|e| format!("Failed to lock the enigo: {}", e))?;
                log::info!("keyboard: {:?} key: {:?}", direction, key);

                enigo
                    .key(key, direction)
                    .map_err(|e| format!("Failed to click key: {e}"))?;

                Ok(())
            })();

            if let Err(e) = tx.send(result) {
                log::error!("Failed to send result from main thread: {e}");
            }
        });

        if let Err(e) = result {
            return Err(format!("Failed to run on main thread: {e}"));
        }

        rx.recv()
            .unwrap_or_else(|e| Err(format!("Failed to receive result: {e}")))
    }

    #[cfg(not(all(target_os = "macos", feature = "tauri")))]
    pub fn keyboard(&self, key: Key, direction: enigo::Direction) -> Result<(), String> {
        let lock = self.enigo.clone();
        let mut enigo = lock
            .lock()
            .map_err(|e| format!("Failed to lock the enigo: {}", e))?;

        log::info!("click keyboard: {:?}", key);

        enigo
            .key(key, direction)
            .map_err(|err| format!("Failed to click: {err}"))?;
        Ok(())
    }

    #[cfg(all(target_os = "macos", feature = "tauri"))]
    pub fn type_values(&self, val: String) -> Result<(), String> {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel::<Result<(), String>>();

        let lock = self.enigo.clone();

        let result = self.app.run_on_main_thread(move || {
            let result = (|| {
                let mut enigo = lock
                    .lock()
                    .map_err(|e| format!("Failed to lock the enigo: {}", e))?;
                log::info!("input text: {}", val);

                enigo
                    .text(&val)
                    .map_err(|e| format!("Failed to click key: {e}"))?;

                Ok(())
            })();

            if let Err(e) = tx.send(result) {
                log::error!("Failed to send result from main thread: {e}");
            }
        });

        if let Err(e) = result {
            return Err(format!("Failed to run on main thread: {e}"));
        }

        rx.recv()
            .unwrap_or_else(|e| Err(format!("Failed to receive result: {e}")))
    }

    #[cfg(not(all(target_os = "macos", feature = "tauri")))]
    pub fn type_values(&self, val: String) -> Result<(), String> {
        let lock = self.enigo.clone();
        let mut enigo = lock
            .lock()
            .map_err(|e| format!("Failed to lock the enigo: {}", e))?;

        enigo
            .text(&val)
            .map_err(|err| format!("Failed to click {val}: {err}"))?;

        Ok(())
    }

    pub fn press_keyboard(&self, val: char, duration: Duration) -> Result<(), String> {
        let mut enigo = Enigo::new(&Default::default()).map_err(|e| e.to_string())?;

        log::info!("press keyboard: {}", val);

        let unicode = Key::Unicode(val);
        enigo
            .key(unicode, Press)
            .map_err(|err| format!("Failed to press: {err}"))?;
        thread::sleep(duration);
        enigo
            .key(unicode, Release)
            .map_err(|err| format!("Failed to release: {err}"))?;

        Ok(())
    }
}
