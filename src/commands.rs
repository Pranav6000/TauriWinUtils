use tauri::State;
use crate::window_manager::{WindowManager, ManagedWindow};
use crate::workspace::Workspace;
use crate::layout::LayoutType;
use crate::config::Config;
use crate::system_window::SystemWindow;

#[tauri::command]
pub fn get_windows(wm: State<WindowManager>) -> Vec<ManagedWindow> {
    wm.get_windows()
}

#[tauri::command]
pub fn add_window_to_manager(
    wm: State<WindowManager>, 
    title: String, 
    app_name: String
) -> Result<String, String> {
    wm.add_window(title, app_name)
}

#[tauri::command]
pub fn remove_window_from_manager(wm: State<WindowManager>, window_id: String) -> Result<(), String> {
    wm.remove_window(&window_id)
}

#[tauri::command]
pub fn create_workspace(wm: State<WindowManager>, name: String, layout: String) -> Result<String, String> {
    let layout_type = match layout.as_str() {
        "tiling" => LayoutType::Tiling,
        "floating" => LayoutType::Floating,
        "monocle" => LayoutType::Monocle,
        _ => return Err("Invalid layout type".to_string()),
    };
    
    Ok(wm.create_workspace(name, layout_type))
}

#[tauri::command]
pub fn switch_workspace(wm: State<WindowManager>, workspace_id: String) -> Result<(), String> {
    wm.switch_workspace(&workspace_id)
}

#[tauri::command]
pub fn get_workspaces(wm: State<WindowManager>) -> Vec<Workspace> {
    wm.get_workspaces()
}

#[tauri::command]
pub fn arrange_windows(wm: State<WindowManager>, workspace_id: String) -> Result<(), String> {
    wm.arrange_workspace(&workspace_id)
}

#[tauri::command]
pub fn close_window(wm: State<WindowManager>, window_id: String) -> Result<(), String> {
    wm.remove_window(&window_id)
}

#[tauri::command]
pub fn minimize_window(wm: State<WindowManager>, window_id: String) -> Result<(), String> {
    // In a real implementation, this would minimize the actual window
    // For now, we'll just update the state
    if let Some(window) = wm.windows.lock().unwrap().get_mut(&window_id) {
        window.state = crate::window_manager::WindowState::Minimized;
        Ok(())
    } else {
        Err("Window not found".to_string())
    }
}

#[tauri::command]
pub fn maximize_window(wm: State<WindowManager>, window_id: String) -> Result<(), String> {
    if let Some(window) = wm.windows.lock().unwrap().get_mut(&window_id) {
        window.state = crate::window_manager::WindowState::Maximized;
        Ok(())
    } else {
        Err("Window not found".to_string())
    }
}

#[tauri::command]
pub fn focus_window(wm: State<WindowManager>, window_id: String) -> Result<(), String> {
    wm.focus_window(&window_id)
}

#[tauri::command]
pub fn get_config(wm: State<WindowManager>) -> Config {
    wm.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_config(wm: State<WindowManager>, config: Config) -> Result<(), String> {
    *wm.config.lock().unwrap() = config;
    Ok(())
}

// System window management commands
#[tauri::command]
pub fn get_system_windows(wm: State<WindowManager>) -> Result<Vec<SystemWindow>, String> {
    wm.get_system_windows()
}

#[tauri::command]
pub fn move_system_window(wm: State<WindowManager>, handle: u64, x: i32, y: i32) -> Result<(), String> {
    wm.move_system_window(handle, x, y)
}

#[tauri::command]
pub fn resize_system_window(wm: State<WindowManager>, handle: u64, width: u32, height: u32) -> Result<(), String> {
    wm.resize_system_window(handle, width, height)
}

#[tauri::command]
pub fn set_system_window_bounds(wm: State<WindowManager>, handle: u64, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
    wm.set_system_window_bounds(handle, x, y, width, height)
}

#[tauri::command]
pub fn minimize_system_window(wm: State<WindowManager>, handle: u64) -> Result<(), String> {
    wm.minimize_system_window(handle)
}

#[tauri::command]
pub fn maximize_system_window(wm: State<WindowManager>, handle: u64) -> Result<(), String> {
    wm.maximize_system_window(handle)
}

#[tauri::command]
pub fn restore_system_window(wm: State<WindowManager>, handle: u64) -> Result<(), String> {
    wm.restore_system_window(handle)
}

#[tauri::command]
pub fn close_system_window(wm: State<WindowManager>, handle: u64) -> Result<(), String> {
    wm.close_system_window(handle)
}

#[tauri::command]
pub fn focus_system_window(wm: State<WindowManager>, handle: u64) -> Result<(), String> {
    wm.focus_system_window(handle)
}

#[tauri::command]
pub fn hide_system_window(wm: State<WindowManager>, handle: u64) -> Result<(), String> {
    wm.hide_system_window(handle)
}

#[tauri::command]
pub fn show_system_window(wm: State<WindowManager>, handle: u64) -> Result<(), String> {
    wm.show_system_window(handle)
}

#[tauri::command]
pub fn arrange_system_windows(wm: State<WindowManager>, window_handles: Vec<u64>) -> Result<(), String> {
    wm.arrange_system_windows(&window_handles)
}