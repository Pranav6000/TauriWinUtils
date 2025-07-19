use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::layout::LayoutType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub layout: LayoutType,
    pub windows: Vec<String>,
    pub focused_window: Option<String>,
}

impl Workspace {
    pub fn new(name: String, layout: LayoutType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            layout,
            windows: Vec::new(),
            focused_window: None,
        }
    }

    pub fn add_window(&mut self, window_id: String) {
        if !self.windows.contains(&window_id) {
            self.windows.push(window_id.clone());
            self.focused_window = Some(window_id);
        }
    }

    pub fn remove_window(&mut self, window_id: &str) {
        self.windows.retain(|id| id != window_id);
        
        if self.focused_window.as_ref() == Some(&window_id.to_string()) {
            self.focused_window = self.windows.last().cloned();
        }
    }

    pub fn focus_window(&mut self, window_id: &str) {
        if self.windows.contains(&window_id.to_string()) {
            self.focused_window = Some(window_id.to_string());
        }
    }

    pub fn get_windows(&self) -> &[String] {
        &self.windows
    }

    pub fn set_layout(&mut self, layout: LayoutType) {
        self.layout = layout;
    }
}