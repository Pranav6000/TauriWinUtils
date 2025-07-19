# Tauri Window Manager Crate

A cross-platform window manager crate for Tauri applications that provides workspace management, window tiling, and layout control.

## Links

- Docs:https://docs.rs/crate/tauri-winutils/latest
- Crate:https://crates.io/crates/tauri-winutils

## Features

- 🖥️ **Real System Window Control**: Actually move, resize, minimize, maximize, and close system windows
- 🌍 **Cross-platform**: Works on Windows, macOS, and Linux
- 🏢 **Workspace Management**: Multiple workspaces with different layouts
- 📐 **Layout Modes**: Tiling, Floating, and Monocle layouts
- ⚙️ **Configurable**: Customizable gaps, borders, and behavior
- 🎯 **Easy Integration**: Simple plugin system for Tauri apps

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tauri-winutils-crate = "0.1.0"
```

## Quick Start

### 1. Add to your Tauri app

```rust
use tauri_winutils_crate;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_winutils_crate::init_window_manager())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Use the convenience macro (alternative)

```rust
use tauri_winutils_crate::setup_window_manager;

fn main() {
    let app = tauri::Builder::default();
    
    setup_window_manager!(app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Frontend Integration

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Get all system windows
const systemWindows = await invoke('get_system_windows');

// Move a window
await invoke('move_system_window', { handle: windowHandle, x: 100, y: 100 });

// Resize a window
await invoke('resize_system_window', { handle: windowHandle, width: 800, height: 600 });

// Minimize/maximize/restore
await invoke('minimize_system_window', { handle: windowHandle });
await invoke('maximize_system_window', { handle: windowHandle });
await invoke('restore_system_window', { handle: windowHandle });

// Close a window
await invoke('close_system_window', { handle: windowHandle });

// Focus a window
await invoke('focus_system_window', { handle: windowHandle });

// Arrange multiple windows in a tiling layout
await invoke('arrange_system_windows', { windowHandles: [handle1, handle2, handle3] });

// Create a new workspace
const workspaceId = await invoke('create_workspace', {
    name: 'Development',
    layout: 'tiling'
});

// Switch to a workspace
await invoke('switch_workspace', { workspaceId });

// Add a window to management
const windowId = await invoke('add_window_to_manager', {
    title: 'My App',
    app_name: 'my-app'
});

// Arrange windows in current workspace
await invoke('arrange_windows', { workspaceId });
```

## Available Commands

### Window Management
- `get_system_windows()` - Get all system windows
- `move_system_window(handle, x, y)` - Move a window
- `resize_system_window(handle, width, height)` - Resize a window
- `set_system_window_bounds(handle, x, y, width, height)` - Set position and size
- `minimize_system_window(handle)` - Minimize a window
- `maximize_system_window(handle)` - Maximize a window
- `restore_system_window(handle)` - Restore a window
- `close_system_window(handle)` - Close a window
- `focus_system_window(handle)` - Focus a window
- `hide_system_window(handle)` - Hide a window
- `show_system_window(handle)` - Show a window
- `arrange_system_windows(handles)` - Arrange multiple windows in a tiling layout

### Virtual Window Management (for internal app windows)
- `add_window_to_manager(title, app_name)` - Add a window to management
- `remove_window_from_manager(window_id)` - Remove a window
- `close_window(window_id)` - Close a window
- `minimize_window(window_id)` - Minimize a window
- `maximize_window(window_id)` - Maximize a window
- `focus_window(window_id)` - Focus a window

### Workspace Management
- `get_workspaces()` - Get all workspaces
- `create_workspace(name, layout)` - Create a new workspace
- `switch_workspace(workspace_id)` - Switch to a workspace
- `arrange_windows(workspace_id)` - Arrange windows in a workspace

### Configuration
- `get_config()` - Get current configuration
- `update_config(config)` - Update configuration

## Layout Types

- **Tiling**: Automatically arranges system windows in a grid layout
- **Floating**: Windows maintain their positions
- **Monocle**: Full-screen window mode

## Configuration Options

```rust
use tauri_winutils_crate::Config;

let config = Config {
    window_gap: 10,
    screen_width: 1920,
    screen_height: 1080,
    auto_arrange: true,
    focus_follows_mouse: false,
    border_width: 2,
    border_color_active: "#0066cc".to_string(),
    border_color_inactive: "#666666".to_string(),
    // ... keybindings
};
```

## Advanced Usage

### Access Window Manager State

```rust
use tauri_winutils_crate::{get_window_manager, WindowManager};

#[tauri::command]
fn my_custom_command(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    if let Some(wm) = get_window_manager(&app_handle) {
        let system_windows = wm.get_system_windows()?;
        let titles: Vec<String> = system_windows.iter().map(|w| w.title.clone()).collect();
        Ok(titles)
    } else {
        Err("Window manager not initialized".to_string())
    }
}
```

### Custom Window Operations

```rust
use tauri_winutils_crate::{WindowManager, SystemWindow};

// In your Tauri command
#[tauri::command]
fn tile_all_windows(wm: tauri::State<WindowManager>) -> Result<(), String> {
    let system_windows = wm.get_system_windows()?;
    let handles: Vec<u64> = system_windows.iter().map(|w| w.handle).collect();
    wm.arrange_system_windows(&handles)?;
    Ok(())
}
```

## Examples

See the `examples/` directory for complete example applications that demonstrate:
- Real system window control
- Automatic window tiling
- Window management UI
- Cross-platform compatibility

## License

MIT License - see LICENSE file for details.
