// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use std::time::Duration;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn list_serial_ports() -> Vec<String> {
    //let mut list = String::new();
    let mut list: Vec<String> = vec![];
    let ports = tokio_serial::available_ports().expect("No ports found!");
    for p in ports {
      list.push(p.port_name);
    }
  
    list
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_serial_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
