use tauri::Manager;

pub fn open_devtools(window: tauri::Window) {
    window.open_devtools();
}
