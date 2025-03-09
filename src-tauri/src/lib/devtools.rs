use tauri::Manager;

#[tauri::command]
pub fn open_devtools(window: tauri::Window) {
    if cfg!(debug_assertions) {
        window.open_devtools();
    }
}

