#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowEvent};

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
    // Here, implement the logic to read the file and send the API request.
    // For simplicity, this is just a placeholder function.
    Ok(format!("File content for: {}", args.file_path))
}
