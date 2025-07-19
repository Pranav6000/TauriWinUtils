pub mod window_manager;
pub mod workspace;
pub mod layout;
pub mod config;
pub mod commands;
pub mod system_window;

pub use window_manager::{WindowManager, ManagedWindow, WindowPosition, WindowSize, WindowState};
pub use workspace::Workspace;
pub use layout::LayoutType;
pub use config::{Config, KeyBindings};
pub use commands::*;
pub use system_window::{SystemWindow, SystemWindowManager, PlatformWindowManager};

use tauri::{Manager, AppHandle, State};

/// Initialize the window manager plugin for a Tauri application
pub fn init_window_manager<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("window-manager")
        .invoke_handler(tauri::generate_handler![
            get_windows,
            create_workspace,
            switch_workspace,
            get_workspaces,
            arrange_windows,
            close_window,
            minimize_window,
            maximize_window,
            focus_window,
            get_config,
            update_config,
            add_window_to_manager,
            remove_window_from_manager,
            get_system_windows,
            move_system_window,
            resize_system_window,
            set_system_window_bounds,
            minimize_system_window,
            maximize_system_window,
            restore_system_window,
            close_system_window,
            focus_system_window,
            hide_system_window,
            show_system_window,
            arrange_system_windows
        ])
        .setup(|app_handle, _| {  // <-- fixed here, 2 args now
            app_handle.manage(WindowManager::new());
            Ok(())
        })
        .build()
}

/// Helper function to get the window manager from app state
pub fn get_window_manager(app_handle: &AppHandle) -> Option<State<WindowManager>> {
    app_handle.try_state::<WindowManager>()
}

/// Convenience macro for easy integration
#[macro_export]
macro_rules! setup_window_manager {
    ($app:expr) => {
        $app.plugin(tauri_window_manager_crate::init_window_manager())
    };
}
