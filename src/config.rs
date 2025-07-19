use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window_gap: u32,
    pub screen_width: u32,
    pub screen_height: u32,
    pub auto_arrange: bool,
    pub focus_follows_mouse: bool,
    pub border_width: u32,
    pub border_color_active: String,
    pub border_color_inactive: String,
    pub keybindings: KeyBindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub switch_workspace_1: String,
    pub switch_workspace_2: String,
    pub switch_workspace_3: String,
    pub switch_workspace_4: String,
    pub close_window: String,
    pub toggle_layout: String,
    pub focus_next: String,
    pub focus_prev: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_gap: 10,
            screen_width: 1920,
            screen_height: 1080,
            auto_arrange: true,
            focus_follows_mouse: false,
            border_width: 2,
            border_color_active: "#0066cc".to_string(),
            border_color_inactive: "#666666".to_string(),
            keybindings: KeyBindings::default(),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            switch_workspace_1: "Super+1".to_string(),
            switch_workspace_2: "Super+2".to_string(),
            switch_workspace_3: "Super+3".to_string(),
            switch_workspace_4: "Super+4".to_string(),
            close_window: "Super+q".to_string(),
            toggle_layout: "Super+space".to_string(),
            focus_next: "Super+j".to_string(),
            focus_prev: "Super+k".to_string(),
        }
    }
}