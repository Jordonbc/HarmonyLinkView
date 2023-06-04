#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::trace;
use log4rs;

use std::env;
use tauri::{Manager};

use crate::globals::*;
mod globals;

use serde::{Deserialize, Serialize};

const SERVER: &str = "http://localhost:9000/";

#[derive(Deserialize, Serialize, Clone)]
struct DeviceInfo {
    os_info: OSInfo,
    battery_info: BatteryInfo,
    dock_info: DockInfo
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub enum DockModel {
    SteamDeckDock,
    JSAUX,
    Unknown,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DockInfo {
    pub dock_model: Option<SDock>,
    pub is_docked: bool,
    pub fallback_detection: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct SDock {
    pub model: DockModel,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ChargingStatus {
    Charging,
    Battery,
    Unknown,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BatteryInfo {
    has_battery: bool,
    battery_percent: i8,
    charging_status: ChargingStatus,
}

#[derive(Deserialize, Serialize, Clone)]
struct VersionData {
    build_timestamp: String,
    git_branch: String,
    git_describe: String,
    git_commit_timestamp: String,
    debug: bool,
    version: String,
    version_major: i32,
    version_minor: i32,
    version_patch: i32,
    version_pre: String
}

impl VersionData {
    fn new() -> VersionData {
        VersionData { build_timestamp: "".to_string(),
        git_branch: "".to_string(),
        git_describe: "".to_string(),
        git_commit_timestamp: "".to_string(),
        debug: true,
        version: "".to_string(),
        version_major: 0,
        version_minor: 0,
        version_patch: 0,
        version_pre: "".to_string() }
    }
}


#[derive(Deserialize, Serialize, Clone)]
struct OSInfo {
    os_type: String,
    os_version: String,
    os_edition: String,
    os_bits: String,
}

fn heartbeat() -> Result<bool, Box<dyn std::error::Error>> {
    let heartbeat = reqwest::blocking::get(SERVER.to_string() + "are_you_there")?
        .text()?;

    Ok(heartbeat == "yes")
}

fn get_os_info() -> Result<OSInfo, Box<dyn std::error::Error>> {
    let info = reqwest::blocking::get(SERVER.to_string() + "os_info")?
    .json()?;

    Ok(info)
}

fn get_battery_info() -> Result<BatteryInfo, Box<dyn std::error::Error>> {
    let info = reqwest::blocking::get(SERVER.to_string() + "battery_info")?
    .json()?;

    Ok(info)
}

fn get_all_info() -> Result<DeviceInfo, Box<dyn std::error::Error>> {
    let info = reqwest::blocking::get(SERVER.to_string() + "all_info")?
    .json()?;

    Ok(info)
}

fn get_version_info() -> Result<VersionData, Box<dyn std::error::Error>> {
    let info = reqwest::blocking::get(SERVER.to_string() + "version_info")?
    .json()?;

    Ok(info)
}

fn refresh() -> Result<(), Box<dyn std::error::Error>> {
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;
    loop {
        match update_frontend() {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        };

        sleep(next_time - Instant::now());
        next_time += interval;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    trace!("Hello, World! I'm awake!");
    
    tauri::Builder::default()
        .setup(|app|{
            set_main_handle(app.app_handle());

            set_main_window(app.get_window("main").unwrap());

            std::thread::spawn(|| {
                if let Err(e) = refresh() {
                    // Log the error, print it, or handle it as necessary.
                    eprintln!("Failed to refresh: {}", e);
                }
            });
            

            //get_main_window().maximize().unwrap();

            Ok(())
        })
        
        .invoke_handler(tauri::generate_handler![
            // tauri commands for frontend here,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
