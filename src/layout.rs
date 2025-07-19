use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    Tiling,
    Floating,
    Monocle,
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::Tiling
    }
}

impl std::fmt::Display for LayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutType::Tiling => write!(f, "Tiling"),
            LayoutType::Floating => write!(f, "Floating"),
            LayoutType::Monocle => write!(f, "Monocle"),
        }
    }
}