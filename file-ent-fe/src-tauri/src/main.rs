#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowEvent};
use std::fs;

#[derive(serde::Deserialize, Debug)]
struct HandleFileArgs {
    file_path: String,
}

fn main() {
    tauri::Builder::default()
        .menu(Menu::new()
            .add_submenu(Submenu::new("File", Menu::new()
                .add_item(CustomMenuItem::new("open", "Open"))
                .add_item(CustomMenuItem::new("exit", "Exit"))))
            .add_submenu(Submenu::new("Edit", Menu::new()
                .add_item(CustomMenuItem::new("undo", "Undo"))))
        )
        .invoke_handler(tauri::generate_handler![handle_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn handle_file(args: HandleFileArgs) -> Result<String, String> {
    match fs::read_to_string(&args.file_path) {
        Ok(contents) => {
            // Now `contents` holds the content of your file as a String.
            Ok(contents)
        },
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}
