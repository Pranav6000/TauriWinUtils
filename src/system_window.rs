use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemWindow {
    pub handle: u64,
    pub title: String,
    pub process_name: String,
    pub pid: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub is_visible: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
}

pub trait SystemWindowManager {
    fn get_all_windows() -> Result<Vec<SystemWindow>, String>;
    fn get_window_by_handle(handle: u64) -> Result<Option<SystemWindow>, String>;
    fn move_window(handle: u64, x: i32, y: i32) -> Result<(), String>;
    fn resize_window(handle: u64, width: u32, height: u32) -> Result<(), String>;
    fn set_window_position_and_size(handle: u64, x: i32, y: i32, width: u32, height: u32) -> Result<(), String>;
    fn minimize_window(handle: u64) -> Result<(), String>;
    fn maximize_window(handle: u64) -> Result<(), String>;
    fn restore_window(handle: u64) -> Result<(), String>;
    fn close_window(handle: u64) -> Result<(), String>;
    fn focus_window(handle: u64) -> Result<(), String>;
    fn hide_window(handle: u64) -> Result<(), String>;
    fn show_window(handle: u64) -> Result<(), String>;
}

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::ptr;
    use winapi::shared::windef::{HWND, RECT};
    use winapi::um::winuser::*;
    use winapi::um::processthreadsapi::GetProcessId;
    use winapi::um::psapi::GetModuleBaseNameW;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::shared::minwindef::{DWORD, MAX_PATH};

    pub struct WindowsManager;

    impl SystemWindowManager for WindowsManager {
        fn get_all_windows() -> Result<Vec<SystemWindow>, String> {
            let mut windows = Vec::new();
            
            unsafe {
                EnumWindows(Some(enum_windows_proc), &mut windows as *mut Vec<SystemWindow> as isize);
            }
            
            Ok(windows)
        }

        fn get_window_by_handle(handle: u64) -> Result<Option<SystemWindow>, String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if IsWindow(hwnd) == 0 {
                    return Ok(None);
                }
                
                let window = get_window_info(hwnd)?;
                Ok(Some(window))
            }
        }

        fn move_window(handle: u64, x: i32, y: i32) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                let mut rect: RECT = std::mem::zeroed();
                if GetWindowRect(hwnd, &mut rect) == 0 {
                    return Err("Failed to get window rect".to_string());
                }
                
                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;
                
                if SetWindowPos(hwnd, ptr::null_mut(), x, y, width, height, SWP_NOZORDER | SWP_NOACTIVATE) == 0 {
                    return Err("Failed to move window".to_string());
                }
            }
            
            Ok(())
        }

        fn resize_window(handle: u64, width: u32, height: u32) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                let mut rect: RECT = std::mem::zeroed();
                if GetWindowRect(hwnd, &mut rect) == 0 {
                    return Err("Failed to get window rect".to_string());
                }
                
                if SetWindowPos(hwnd, ptr::null_mut(), rect.left, rect.top, width as i32, height as i32, SWP_NOZORDER | SWP_NOACTIVATE) == 0 {
                    return Err("Failed to resize window".to_string());
                }
            }
            
            Ok(())
        }

        fn set_window_position_and_size(handle: u64, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if SetWindowPos(hwnd, ptr::null_mut(), x, y, width as i32, height as i32, SWP_NOZORDER | SWP_NOACTIVATE) == 0 {
                    return Err("Failed to set window position and size".to_string());
                }
            }
            
            Ok(())
        }

        fn minimize_window(handle: u64) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if ShowWindow(hwnd, SW_MINIMIZE) == 0 {
                    return Err("Failed to minimize window".to_string());
                }
            }
            
            Ok(())
        }

        fn maximize_window(handle: u64) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if ShowWindow(hwnd, SW_MAXIMIZE) == 0 {
                    return Err("Failed to maximize window".to_string());
                }
            }
            
            Ok(())
        }

        fn restore_window(handle: u64) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if ShowWindow(hwnd, SW_RESTORE) == 0 {
                    return Err("Failed to restore window".to_string());
                }
            }
            
            Ok(())
        }

        fn close_window(handle: u64) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if PostMessageW(hwnd, WM_CLOSE, 0, 0) == 0 {
                    return Err("Failed to close window".to_string());
                }
            }
            
            Ok(())
        }

        fn focus_window(handle: u64) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if SetForegroundWindow(hwnd) == 0 {
                    return Err("Failed to focus window".to_string());
                }
            }
            
            Ok(())
        }

        fn hide_window(handle: u64) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if ShowWindow(hwnd, SW_HIDE) == 0 {
                    return Err("Failed to hide window".to_string());
                }
            }
            
            Ok(())
        }

        fn show_window(handle: u64) -> Result<(), String> {
            let hwnd = handle as HWND;
            
            unsafe {
                if ShowWindow(hwnd, SW_SHOW) == 0 {
                    return Err("Failed to show window".to_string());
                }
            }
            
            Ok(())
        }
    }

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> i32 {
        let windows = &mut *(lparam as *mut Vec<SystemWindow>);
        
        if IsWindowVisible(hwnd) != 0 {
            if let Ok(window) = get_window_info(hwnd) {
                if !window.title.is_empty() {
                    windows.push(window);
                }
            }
        }
        
        1 // Continue enumeration
    }

    unsafe fn get_window_info(hwnd: HWND) -> Result<SystemWindow, String> {
        let mut title_buf = [0u16; 256];
        let title_len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32);
        let title = if title_len > 0 {
            OsString::from_wide(&title_buf[..title_len as usize])
                .to_string_lossy()
                .to_string()
        } else {
            String::new()
        };

        let mut rect: RECT = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect);

        let pid = GetProcessId(hwnd as *mut _);
        let process_name = get_process_name(pid).unwrap_or_else(|| "Unknown".to_string());

        let placement = {
            let mut wp: WINDOWPLACEMENT = std::mem::zeroed();
            wp.length = std::mem::size_of::<WINDOWPLACEMENT>() as u32;
            GetWindowPlacement(hwnd, &mut wp);
            wp
        };

        Ok(SystemWindow {
            handle: hwnd as u64,
            title,
            process_name,
            pid,
            x: rect.left,
            y: rect.top,
            width: (rect.right - rect.left) as u32,
            height: (rect.bottom - rect.top) as u32,
            is_visible: IsWindowVisible(hwnd) != 0,
            is_minimized: placement.showCmd == SW_SHOWMINIMIZED as u32,
            is_maximized: placement.showCmd == SW_SHOWMAXIMIZED as u32,
        })
    }

    unsafe fn get_process_name(pid: DWORD) -> Option<String> {
        let process_handle = OpenProcess(0x0400 | 0x0010, 0, pid); // PROCESS_QUERY_INFORMATION | PROCESS_VM_READ
        if process_handle.is_null() {
            return None;
        }

        let mut name_buf = [0u16; MAX_PATH];
        let name_len = GetModuleBaseNameW(process_handle, ptr::null_mut(), name_buf.as_mut_ptr(), MAX_PATH as u32);
        
        CloseHandle(process_handle);

        if name_len > 0 {
            let name = OsString::from_wide(&name_buf[..name_len as usize])
                .to_string_lossy()
                .to_string();
            Some(name)
        } else {
            None
        }
    }
}

#[cfg(target_os = "macos")]
mod macos_impl {
    use super::*;
    use cocoa::appkit::*;
    use cocoa::base::{id, nil, YES, NO, BOOL};
    use cocoa::foundation::{NSArray, NSString, NSAutoreleasePool, NSDictionary, NSNumber};
    use objc::runtime::Object;
    use objc::{msg_send, sel, sel_impl};
    use core_graphics::geometry::{CGPoint, CGSize, CGRect};
    use core_graphics::window::{CGWindowListOption, CGWindowID, kCGWindowListOptionOnScreenOnly, kCGWindowListExcludeDesktopElements};
    use core_graphics::display::CGDisplay;
    use std::collections::HashMap;
    use std::ffi::{CStr, CString};
    use std::ptr;

    pub struct MacOSManager;

    impl SystemWindowManager for MacOSManager {
        fn get_all_windows() -> Result<Vec<SystemWindow>, String> {
            unsafe {
                let window_list_info = core_graphics::window::CGWindowListCopyWindowInfo(
                    kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements,
                    0
                );
                
                if window_list_info.is_null() {
                    return Err("Failed to get window list".to_string());
                }
                
                let mut windows = Vec::new();
                let count: i64 = msg_send![window_list_info, count];
                
                for i in 0..count {
                    let window_info: id = msg_send![window_list_info, objectAtIndex: i];
                    if let Ok(window) = parse_window_info(window_info) {
                        windows.push(window);
                    }
                }
                
                Ok(windows)
            }
        }

        fn get_window_by_handle(handle: u64) -> Result<Option<SystemWindow>, String> {
            let windows = Self::get_all_windows()?;
            Ok(windows.into_iter().find(|w| w.handle == handle))
        }

        fn move_window(handle: u64, x: i32, y: i32) -> Result<(), String> {
            unsafe {
                let window_id = handle as CGWindowID;
                let app = NSApp();
                let windows: id = msg_send![app, windows];
                let count: usize = msg_send![windows, count];
                
                for i in 0..count {
                    let window: id = msg_send![windows, objectAtIndex: i];
                    let window_number: i32 = msg_send![window, windowNumber];
                    
                    if window_number as u64 == handle {
                        let point = NSPoint::new(x as f64, y as f64);
                        let _: () = msg_send![window, setFrameOrigin: point];
                        return Ok(());
                    }
                }
            }
            Ok(())
        }

        fn resize_window(handle: u64, width: u32, height: u32) -> Result<(), String> {
            unsafe {
                let app = NSApp();
                let windows: id = msg_send![app, windows];
                let count: usize = msg_send![windows, count];
                
                for i in 0..count {
                    let window: id = msg_send![windows, objectAtIndex: i];
                    let window_number: i32 = msg_send![window, windowNumber];
                    
                    if window_number as u64 == handle {
                        let current_frame: NSRect = msg_send![window, frame];
                        let new_frame = NSRect::new(
                            current_frame.origin,
                            NSSize::new(width as f64, height as f64)
                        );
                        let _: () = msg_send![window, setFrame: new_frame display: YES];
                        return Ok(());
                    }
                }
            }
            Ok(())
        }

        fn set_window_position_and_size(handle: u64, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
            unsafe {
                let app = NSApp();
                let windows: id = msg_send![app, windows];
                let count: usize = msg_send![windows, count];
                
                for i in 0..count {
                    let window: id = msg_send![windows, objectAtIndex: i];
                    let window_number: i32 = msg_send![window, windowNumber];
                    
                    if window_number as u64 == handle {
                        let new_frame = NSRect::new(
                            NSPoint::new(x as f64, y as f64),
                            NSSize::new(width as f64, height as f64)
                        );
                        let _: () = msg_send![window, setFrame: new_frame display: YES];
                        return Ok(());
                    }
                }
            }
            Ok(())
        }

        fn minimize_window(handle: u64) -> Result<(), String> {
            unsafe {
                if let Some(window) = find_window_by_handle(handle) {
                    let _: () = msg_send![window, miniaturize: nil];
                }
            }
            Ok(())
        }

        fn maximize_window(handle: u64) -> Result<(), String> {
            unsafe {
                if let Some(window) = find_window_by_handle(handle) {
                    let _: () = msg_send![window, zoom: nil];
                }
            }
            Ok(())
        }

        fn restore_window(handle: u64) -> Result<(), String> {
            unsafe {
                if let Some(window) = find_window_by_handle(handle) {
                    let is_miniaturized: BOOL = msg_send![window, isMiniaturized];
                    if is_miniaturized == YES {
                        let _: () = msg_send![window, deminiaturize: nil];
                    }
                }
            }
            Ok(())
        }

        fn close_window(handle: u64) -> Result<(), String> {
            unsafe {
                if let Some(window) = find_window_by_handle(handle) {
                    let _: () = msg_send![window, close];
                }
            }
            Ok(())
        }

        fn focus_window(handle: u64) -> Result<(), String> {
            unsafe {
                if let Some(window) = find_window_by_handle(handle) {
                    let _: () = msg_send![window, makeKeyAndOrderFront: nil];
                    let app = NSApp();
                    let _: () = msg_send![app, activateIgnoringOtherApps: YES];
                }
            }
            Ok(())
        }

        fn hide_window(handle: u64) -> Result<(), String> {
            unsafe {
                if let Some(window) = find_window_by_handle(handle) {
                    let _: () = msg_send![window, orderOut: nil];
                }
            }
            Ok(())
        }

        fn show_window(handle: u64) -> Result<(), String> {
            unsafe {
                if let Some(window) = find_window_by_handle(handle) {
                    let _: () = msg_send![window, orderFront: nil];
                }
            }
            Ok(())
        }
    }
    
    unsafe fn find_window_by_handle(handle: u64) -> Option<id> {
        let app = NSApp();
        let windows: id = msg_send![app, windows];
        let count: usize = msg_send![windows, count];
        
        for i in 0..count {
            let window: id = msg_send![windows, objectAtIndex: i];
            let window_number: i32 = msg_send![window, windowNumber];
            
            if window_number as u64 == handle {
                return Some(window);
            }
        }
        None
    }
    
    unsafe fn parse_window_info(window_info: id) -> Result<SystemWindow, String> {
        let window_id_key = NSString::alloc(nil).init_str("kCGWindowNumber");
        let window_id_obj: id = msg_send![window_info, objectForKey: window_id_key];
        let window_id: u64 = if !window_id_obj.is_null() {
            let id_value: i64 = msg_send![window_id_obj, longLongValue];
            id_value as u64
        } else {
            return Err("No window ID".to_string());
        };
        
        let name_key = NSString::alloc(nil).init_str("kCGWindowName");
        let name_obj: id = msg_send![window_info, objectForKey: name_key];
        let title = if !name_obj.is_null() {
            let c_str: *const i8 = msg_send![name_obj, UTF8String];
            if !c_str.is_null() {
                CStr::from_ptr(c_str).to_string_lossy().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        let owner_name_key = NSString::alloc(nil).init_str("kCGWindowOwnerName");
        let owner_name_obj: id = msg_send![window_info, objectForKey: owner_name_key];
        let process_name = if !owner_name_obj.is_null() {
            let c_str: *const i8 = msg_send![owner_name_obj, UTF8String];
            if !c_str.is_null() {
                CStr::from_ptr(c_str).to_string_lossy().to_string()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        };
        
        let bounds_key = NSString::alloc(nil).init_str("kCGWindowBounds");
        let bounds_obj: id = msg_send![window_info, objectForKey: bounds_key];
        let (x, y, width, height) = if !bounds_obj.is_null() {
            // Parse CGRect from dictionary
            let x_key = NSString::alloc(nil).init_str("X");
            let y_key = NSString::alloc(nil).init_str("Y");
            let width_key = NSString::alloc(nil).init_str("Width");
            let height_key = NSString::alloc(nil).init_str("Height");
            
            let x_obj: id = msg_send![bounds_obj, objectForKey: x_key];
            let y_obj: id = msg_send![bounds_obj, objectForKey: y_key];
            let width_obj: id = msg_send![bounds_obj, objectForKey: width_key];
            let height_obj: id = msg_send![bounds_obj, objectForKey: height_key];
            
            let x: f64 = if !x_obj.is_null() { msg_send![x_obj, doubleValue] } else { 0.0 };
            let y: f64 = if !y_obj.is_null() { msg_send![y_obj, doubleValue] } else { 0.0 };
            let width: f64 = if !width_obj.is_null() { msg_send![width_obj, doubleValue] } else { 0.0 };
            let height: f64 = if !height_obj.is_null() { msg_send![height_obj, doubleValue] } else { 0.0 };
            
            (x as i32, y as i32, width as u32, height as u32)
        } else {
            (0, 0, 0, 0)
        };
        
        let pid_key = NSString::alloc(nil).init_str("kCGWindowOwnerPID");
        let pid_obj: id = msg_send![window_info, objectForKey: pid_key];
        let pid: u32 = if !pid_obj.is_null() {
            let pid_value: i32 = msg_send![pid_obj, intValue];
            pid_value as u32
        } else {
            0
        };
        
        Ok(SystemWindow {
            handle: window_id,
            title,
            process_name,
            pid,
            x,
            y,
            width,
            height,
            is_visible: true, // Assume visible since we're getting on-screen windows
            is_minimized: false, // Would need additional checks
            is_maximized: false, // Would need additional checks
        })
    }
}

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::*;
    use x11::xlib::*;
    use std::ptr;
    use std::ffi::{CString, CStr};
    use std::mem;

    pub struct LinuxManager;

    impl SystemWindowManager for LinuxManager {
        fn get_all_windows() -> Result<Vec<SystemWindow>, String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let root = XDefaultRootWindow(display);
                let mut windows = Vec::new();
                
                // Get all windows
                let mut root_return = 0;
                let mut parent_return = 0;
                let mut children_return = ptr::null_mut();
                let mut nchildren_return = 0;
                
                if XQueryTree(display, root, &mut root_return, &mut parent_return, 
                             &mut children_return, &mut nchildren_return) != 0 {
                    
                    let children = std::slice::from_raw_parts(children_return, nchildren_return as usize);
                    
                    for &window in children {
                        if let Ok(sys_window) = get_window_info(display, window) {
                            if !sys_window.title.is_empty() && sys_window.is_visible {
                                windows.push(sys_window);
                            }
                        }
                    }
                    
                    XFree(children_return as *mut _);
                }
                
                XCloseDisplay(display);
                Ok(windows)
            }
        }

        fn get_window_by_handle(handle: u64) -> Result<Option<SystemWindow>, String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Ok(None);
                }
                
                let window = handle as Window;
                let result = get_window_info(display, window);
                XCloseDisplay(display);
                
                match result {
                    Ok(window_info) => Ok(Some(window_info)),
                    Err(_) => Ok(None),
                }
            }
        }

        fn move_window(handle: u64, x: i32, y: i32) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XMoveWindow(display, window, x, y);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn resize_window(handle: u64, width: u32, height: u32) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XResizeWindow(display, window, width, height);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn set_window_position_and_size(handle: u64, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XMoveResizeWindow(display, window, x, y, width, height);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn minimize_window(handle: u64) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XIconifyWindow(display, window, XDefaultScreen(display));
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn maximize_window(handle: u64) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                let screen = XDefaultScreen(display);
                let screen_width = XDisplayWidth(display, screen) as u32;
                let screen_height = XDisplayHeight(display, screen) as u32;
                
                XMoveResizeWindow(display, window, 0, 0, screen_width, screen_height);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn restore_window(handle: u64) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XMapWindow(display, window);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn close_window(handle: u64) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                
                // Try to close gracefully first
                let wm_delete_window = XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as *const i8, 0);
                let wm_protocols = XInternAtom(display, b"WM_PROTOCOLS\0".as_ptr() as *const i8, 0);
                
                let mut event: XEvent = mem::zeroed();
                event.client_message.type_ = ClientMessage;
                event.client_message.window = window;
                event.client_message.message_type = wm_protocols;
                event.client_message.format = 32;
                event.client_message.data.set_long(0, wm_delete_window as i64);
                
                XSendEvent(display, window, 0, NoEventMask, &mut event);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn focus_window(handle: u64) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XRaiseWindow(display, window);
                XSetInputFocus(display, window, RevertToParent, CurrentTime);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn hide_window(handle: u64) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XUnmapWindow(display, window);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }

        fn show_window(handle: u64) -> Result<(), String> {
            unsafe {
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err("Cannot open X11 display".to_string());
                }
                
                let window = handle as Window;
                XMapWindow(display, window);
                XFlush(display);
                XCloseDisplay(display);
            }
            Ok(())
        }
    }
    
    unsafe fn get_window_info(display: *mut Display, window: Window) -> Result<SystemWindow, String> {
        let mut attrs: XWindowAttributes = mem::zeroed();
        if XGetWindowAttributes(display, window, &mut attrs) == 0 {
            return Err("Failed to get window attributes".to_string());
        }
        
        // Get window title
        let mut window_name = ptr::null_mut();
        let title = if XFetchName(display, window, &mut window_name) != 0 && !window_name.is_null() {
            let c_str = CStr::from_ptr(window_name);
            let title = c_str.to_string_lossy().to_string();
            XFree(window_name as *mut _);
            title
        } else {
            String::new()
        };
        
        // Get window class (process name)
        let mut class_hint: XClassHint = mem::zeroed();
        let process_name = if XGetClassHint(display, window, &mut class_hint) != 0 {
            let name = if !class_hint.res_class.is_null() {
                CStr::from_ptr(class_hint.res_class).to_string_lossy().to_string()
            } else if !class_hint.res_name.is_null() {
                CStr::from_ptr(class_hint.res_name).to_string_lossy().to_string()
            } else {
                "Unknown".to_string()
            };
            
            if !class_hint.res_name.is_null() {
                XFree(class_hint.res_name as *mut _);
            }
            if !class_hint.res_class.is_null() {
                XFree(class_hint.res_class as *mut _);
            }
            
            name
        } else {
            "Unknown".to_string()
        };
        
        // Get window position relative to root
        let mut x_return = 0;
        let mut y_return = 0;
        let mut child_return = 0;
        XTranslateCoordinates(display, window, attrs.root, 0, 0, &mut x_return, &mut y_return, &mut child_return);
        
        Ok(SystemWindow {
            handle: window as u64,
            title,
            process_name,
            pid: 0, // Would need additional system calls to get PID
            x: x_return,
            y: y_return,
            width: attrs.width as u32,
            height: attrs.height as u32,
            is_visible: attrs.map_state == IsViewable,
            is_minimized: attrs.map_state == IsUnmapped,
            is_maximized: false, // Would need additional checks
        })
    }
}

// Platform-specific type alias
#[cfg(windows)]
pub type PlatformWindowManager = windows_impl::WindowsManager;

#[cfg(target_os = "macos")]
pub type PlatformWindowManager = macos_impl::MacOSManager;

#[cfg(target_os = "linux")]
pub type PlatformWindowManager = linux_impl::LinuxManager;