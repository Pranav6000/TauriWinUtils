// Example: Basic usage of the window manager crate
use tauri_winutils_crate;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_winutils_crate::init_window_manager())
        .invoke_handler(tauri::generate_handler![custom_window_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn custom_window_command(
    wm: tauri::State<tauri_winutils_crate::WindowManager>
) -> Result<usize, String> {
    let windows = wm.get_windows();
    Ok(windows.len())
}