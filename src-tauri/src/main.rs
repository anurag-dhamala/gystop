#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod sys_info;
mod tauri_commands;

use crate::tauri_commands::{get_cpu, get_devices, get_disks, get_memory, get_temperature};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_cpu,
            get_devices,
            get_disks,
            get_memory,
            get_temperature
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
