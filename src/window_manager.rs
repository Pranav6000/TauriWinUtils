use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::workspace::Workspace;
use crate::layout::LayoutType;
use crate::config::Config;
use crate::system_window::{SystemWindow, SystemWindowManager, PlatformWindowManager};

use tauri::command;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedWindow {
    pub id: String,
    pub title: String,
    pub app_name: String,
    pub workspace_id: String,
    pub position: WindowPosition,
    pub size: WindowSize,
    pub state: WindowState,
    pub created_at: DateTime<Utc>,
    pub last_focused: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

pub struct WindowManager {
    pub windows: Arc<Mutex<HashMap<String, ManagedWindow>>>,
    pub workspaces: Arc<Mutex<HashMap<String, Workspace>>>,
    pub active_workspace: Arc<Mutex<String>>,
    pub config: Arc<Mutex<Config>>,
    pub system_windows: Arc<Mutex<HashMap<u64, SystemWindow>>>,
}

impl WindowManager {
    pub fn new() -> Self {
        let mut workspaces = HashMap::new();
        let default_workspace_id = Uuid::new_v4().to_string();
        
        workspaces.insert(
            default_workspace_id.clone(),
            Workspace::new("Default".to_string(), LayoutType::Tiling)
        );

        Self {
            windows: Arc::new(Mutex::new(HashMap::new())),
            workspaces: Arc::new(Mutex::new(workspaces)),
            active_workspace: Arc::new(Mutex::new(default_workspace_id)),
            config: Arc::new(Mutex::new(Config::default())),
            system_windows: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_window(&self, title: String, app_name: String) -> Result<String, String> {
        let window_id = Uuid::new_v4().to_string();
        let active_workspace = self.active_workspace.lock().unwrap().clone();
        
        let window = ManagedWindow {
            id: window_id.clone(),
            title,
            app_name,
            workspace_id: active_workspace.clone(),
            position: WindowPosition { x: 0, y: 0 },
            size: WindowSize { width: 800, height: 600 },
            state: WindowState::Normal,
            created_at: Utc::now(),
            last_focused: Utc::now(),
        };

        self.windows.lock().unwrap().insert(window_id.clone(), window);
        
        if let Some(workspace) = self.workspaces.lock().unwrap().get_mut(&active_workspace) {
            workspace.add_window(window_id.clone());
        }

        self.arrange_workspace(&active_workspace)?;
        
        Ok(window_id)
    }

    pub fn remove_window(&self, window_id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.lock().unwrap().remove(window_id) {
            if let Some(workspace) = self.workspaces.lock().unwrap().get_mut(&window.workspace_id) {
                workspace.remove_window(window_id);
                self.arrange_workspace(&window.workspace_id)?;
            }
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    pub fn get_windows(&self) -> Vec<ManagedWindow> {
        self.windows.lock().unwrap().values().cloned().collect()
    }

    pub fn get_workspace_windows(&self, workspace_id: &str) -> Vec<ManagedWindow> {
        self.windows
            .lock()
            .unwrap()
            .values()
            .filter(|w| w.workspace_id == workspace_id)
            .cloned()
            .collect()
    }

    pub fn focus_window(&self, window_id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.lock().unwrap().get_mut(window_id) {
            window.last_focused = Utc::now();
            
            if let Some(workspace) = self.workspaces.lock().unwrap().get_mut(&window.workspace_id) {
                workspace.focus_window(window_id);
            }
            
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    pub fn arrange_workspace(&self, workspace_id: &str) -> Result<(), String> {
        let workspace = self.workspaces.lock().unwrap();
        let workspace = workspace.get(workspace_id).ok_or("Workspace not found")?;
        
        let window_ids = workspace.get_windows();
        let layout = workspace.layout.clone();
        
        drop(workspace);

        let config = self.config.lock().unwrap();
        let screen_width = config.screen_width;
        let screen_height = config.screen_height;
        let gap = config.window_gap;
        drop(config);

        match layout {
            LayoutType::Tiling => self.arrange_tiling(&window_ids, screen_width, screen_height, gap),
            LayoutType::Floating => Ok(()),
            LayoutType::Monocle => self.arrange_monocle(&window_ids, screen_width, screen_height),
        }
    }

    fn arrange_tiling(&self, window_ids: &[String], screen_width: u32, screen_height: u32, gap: u32) -> Result<(), String> {
        if window_ids.is_empty() {
            return Ok(());
        }

        let mut windows = self.windows.lock().unwrap();
        let count = window_ids.len();
        
        if count == 1 {
            if let Some(window) = windows.get_mut(&window_ids[0]) {
                window.position = WindowPosition { x: gap as i32, y: gap as i32 };
                window.size = WindowSize {
                    width: screen_width - (gap * 2),
                    height: screen_height - (gap * 2),
                };
            }
        } else {
            let cols = (count as f64).sqrt().ceil() as usize;
            let rows = (count + cols - 1) / cols;
            
            let window_width = (screen_width - gap * (cols as u32 + 1)) / cols as u32;
            let window_height = (screen_height - gap * (rows as u32 + 1)) / rows as u32;

            for (i, window_id) in window_ids.iter().enumerate() {
                if let Some(window) = windows.get_mut(window_id) {
                    let col = i % cols;
                    let row = i / cols;
                    
                    window.position = WindowPosition {
                        x: (gap + col as u32 * (window_width + gap)) as i32,
                        y: (gap + row as u32 * (window_height + gap)) as i32,
                    };
                    window.size = WindowSize {
                        width: window_width,
                        height: window_height,
                    };
                }
            }
        }

        Ok(())
    }

    fn arrange_monocle(&self, window_ids: &[String], screen_width: u32, screen_height: u32) -> Result<(), String> {
        let mut windows = self.windows.lock().unwrap();
        
        for window_id in window_ids {
            if let Some(window) = windows.get_mut(window_id) {
                window.position = WindowPosition { x: 0, y: 0 };
                window.size = WindowSize {
                    width: screen_width,
                    height: screen_height,
                };
            }
        }

        Ok(())
    }

    pub fn create_workspace(&self, name: String, layout: LayoutType) -> String {
        let workspace_id = Uuid::new_v4().to_string();
        let workspace = Workspace::new(name, layout);
        
        self.workspaces.lock().unwrap().insert(workspace_id.clone(), workspace);
        workspace_id
    }

    pub fn switch_workspace(&self, workspace_id: &str) -> Result<(), String> {
        if self.workspaces.lock().unwrap().contains_key(workspace_id) {
            *self.active_workspace.lock().unwrap() = workspace_id.to_string();
            Ok(())
        } else {
            Err("Workspace not found".to_string())
        }
    }

    pub fn get_workspaces(&self) -> Vec<Workspace> {
        self.workspaces.lock().unwrap().values().cloned().collect()
    }

    pub fn get_active_workspace(&self) -> String {
        self.active_workspace.lock().unwrap().clone()
    }

    // System window management methods
    pub fn get_system_windows(&self) -> Result<Vec<SystemWindow>, String> {
        let windows = PlatformWindowManager::get_all_windows()?;
        *self.system_windows.lock().unwrap() = windows.iter()
            .map(|w| (w.handle, w.clone()))
            .collect();
        Ok(windows)
    }

    pub fn move_system_window(&self, handle: u64, x: i32, y: i32) -> Result<(), String> {
        PlatformWindowManager::move_window(handle, x, y)?;
        
        if let Ok(Some(window)) = PlatformWindowManager::get_window_by_handle(handle) {
            self.system_windows.lock().unwrap().insert(handle, window);
        }
        
        Ok(())
    }

    pub fn resize_system_window(&self, handle: u64, width: u32, height: u32) -> Result<(), String> {
        PlatformWindowManager::resize_window(handle, width, height)?;
        
        if let Ok(Some(window)) = PlatformWindowManager::get_window_by_handle(handle) {
            self.system_windows.lock().unwrap().insert(handle, window);
        }
        
        Ok(())
    }

    pub fn set_system_window_bounds(&self, handle: u64, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
        PlatformWindowManager::set_window_position_and_size(handle, x, y, width, height)?;
        
        if let Ok(Some(window)) = PlatformWindowManager::get_window_by_handle(handle) {
            self.system_windows.lock().unwrap().insert(handle, window);
        }
        
        Ok(())
    }

    pub fn minimize_system_window(&self, handle: u64) -> Result<(), String> {
        PlatformWindowManager::minimize_window(handle)
    }

    pub fn maximize_system_window(&self, handle: u64) -> Result<(), String> {
        PlatformWindowManager::maximize_window(handle)
    }

    pub fn restore_system_window(&self, handle: u64) -> Result<(), String> {
        PlatformWindowManager::restore_window(handle)
    }

    pub fn close_system_window(&self, handle: u64) -> Result<(), String> {
        PlatformWindowManager::close_window(handle)?;
        self.system_windows.lock().unwrap().remove(&handle);
        Ok(())
    }

    pub fn focus_system_window(&self, handle: u64) -> Result<(), String> {
        PlatformWindowManager::focus_window(handle)
    }

    pub fn hide_system_window(&self, handle: u64) -> Result<(), String> {
        PlatformWindowManager::hide_window(handle)
    }

    pub fn show_system_window(&self, handle: u64) -> Result<(), String> {
        PlatformWindowManager::show_window(handle)
    }

    pub fn arrange_system_windows(&self, window_handles: &[u64]) -> Result<(), String> {
        let config = self.config.lock().unwrap();
        let screen_width = config.screen_width;
        let screen_height = config.screen_height;
        let gap = config.window_gap;
        drop(config);

        if window_handles.is_empty() {
            return Ok(());
        }

        let count = window_handles.len();
        
        if count == 1 {
            self.set_system_window_bounds(
                window_handles[0],
                gap as i32,
                gap as i32,
                screen_width - (gap * 2),
                screen_height - (gap * 2)
            )?;
        } else {
            let cols = (count as f64).sqrt().ceil() as usize;
            let rows = (count + cols - 1) / cols;
            
            let window_width = (screen_width - gap * (cols as u32 + 1)) / cols as u32;
            let window_height = (screen_height - gap * (rows as u32 + 1)) / rows as u32;

            for (i, &handle) in window_handles.iter().enumerate() {
                let col = i % cols;
                let row = i / cols;
                
                let x = (gap + col as u32 * (window_width + gap)) as i32;
                let y = (gap + row as u32 * (window_height + gap)) as i32;
                
                self.set_system_window_bounds(handle, x, y, window_width, window_height)?;
            }
        }

        Ok(())
    }
}

// === Tauri integration part ===

// Global WindowManager singleton instance:
static WINDOW_MANAGER: Lazy<Mutex<WindowManager>> = Lazy::new(|| Mutex::new(WindowManager::new()));

#[command]
pub fn get_system_windows() -> Result<Vec<SystemWindow>, String> {
    let wm = WINDOW_MANAGER.lock().unwrap();
    wm.get_system_windows()
}

#[command]
pub fn focus_system_window(handle: u64) -> Result<(), String> {
    let mut wm = WINDOW_MANAGER.lock().unwrap();
    wm.focus_system_window(handle)
}

pub fn init() -> tauri::plugin::TauriPlugin {
    tauri::plugin::Builder::new("tauri-winutils")
        .invoke_handler(tauri::generate_handler![
            get_system_windows,
            focus_system_window,
        ])
        .build()
}
