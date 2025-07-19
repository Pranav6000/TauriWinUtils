// Example: Advanced usage with custom workspace management
use tauri_winutils_crate::{WindowManager, LayoutType, get_window_manager};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_winutils_crate::init_window_manager())
        .invoke_handler(tauri::generate_handler![
            setup_development_workspace,
            get_workspace_stats
        ])
        .setup(|app| {
            // Initialize with custom workspaces
            if let Some(wm) = get_window_manager(app.handle()) {
                wm.create_workspace("Development".to_string(), LayoutType::Tiling);
                wm.create_workspace("Communication".to_string(), LayoutType::Floating);
                wm.create_workspace("Media".to_string(), LayoutType::Monocle);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn setup_development_workspace(wm: tauri::State<WindowManager>) -> Result<String, String> {
    let workspace_id = wm.create_workspace("Dev Environment".to_string(), LayoutType::Tiling);
    
    // Add some mock windows for demonstration
    wm.add_window("VS Code".to_string(), "code".to_string())?;
    wm.add_window("Terminal".to_string(), "terminal".to_string())?;
    wm.add_window("Browser".to_string(), "firefox".to_string())?;
    
    wm.arrange_workspace(&workspace_id)?;
    
    Ok(workspace_id)
}

#[tauri::command]
fn get_workspace_stats(wm: tauri::State<WindowManager>) -> Result<serde_json::Value, String> {
    let workspaces = wm.get_workspaces();
    let windows = wm.get_windows();
    
    let stats = serde_json::json!({
        "total_workspaces": workspaces.len(),
        "total_windows": windows.len(),
        "active_workspace": wm.get_active_workspace(),
        "workspaces": workspaces.iter().map(|w| {
            serde_json::json!({
                "id": w.id,
                "name": w.name,
                "layout": w.layout,
                "window_count": w.windows.len()
            })
        }).collect::<Vec<_>>()
    });
    
    Ok(stats)
}