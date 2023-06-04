use log::{trace, info, error};
use tauri::Manager;
use std::sync::{Mutex, mpsc::RecvError};

use crate::{heartbeat, DeviceInfo, get_all_info};

#[derive(Debug, Clone)]
pub struct Window {
    pub window: tauri::Window
}

pub static MAIN_HANDLE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);
pub static MAIN_WINDOW: Mutex<Option<Window>> = Mutex::new(None);

pub fn get_window(window_label: &str) -> Option<tauri::Window> {
    trace!("getting current window reference");

    MAIN_HANDLE.lock().unwrap().clone().unwrap().get_window(window_label)
}

pub fn set_main_handle(app_handle: tauri::AppHandle) {
    *MAIN_HANDLE.lock().unwrap() = Some(app_handle);
}

pub fn get_main_window() -> tauri::Window {
    get_window("main").unwrap()
}

pub fn set_main_window(new_window: tauri::Window) {
    trace!("Setting main Window: {}", new_window.label());
    *MAIN_WINDOW.lock().unwrap() = Some(Window { window: new_window });
}

pub fn update_frontend() -> Result<(), Box<dyn std::error::Error>> {
    info!("Updating frontend!");

    let result = match get_all_info() {
        Ok(data) => Ok(data),
        Err(e) => {
            error!("Error getting all info: {}", e);
            Err(e.to_string())  // Convert the error to a string so it can be sent to the frontend
        }
    };

    // Emit the "update_frontend" event regardless of whether an error occurred
    // If an error occurred, result will be Err(e) where e is a string
    // If no error occurred, result will be Ok(data)
    get_main_window().emit_all("update_frontend", result.clone())
        .map_err(|e| {
            error!("Error sending 'update_frontend' event to frontend: {}", e);
            e.into()  // Convert e into a Box<dyn Error>
        })
}

