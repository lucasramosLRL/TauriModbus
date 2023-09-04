// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn list_serial_ports() -> String {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
      println!("{}", p.port_name);
    }
    return "Isso é realmente um teste".into();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_serial_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
