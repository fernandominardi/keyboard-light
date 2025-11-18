use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Use Tauri's API to make the window ignore mouse events.
            window.set_ignore_cursor_events(true).unwrap();

            // Configure window to be click-through (allows mouse events to pass through to underlying windows)
            #[cfg(target_os = "windows")]
            {
                use windows::Win32::Foundation::HWND;
                use windows::Win32::UI::WindowsAndMessaging::{
                    GetWindowLongW, SetWindowLongW, GWL_EXSTYLE,
                    WS_EX_LAYERED, WS_EX_TRANSPARENT, WS_EX_NOACTIVATE
                };

                let hwnd = HWND(window.hwnd().unwrap().0);
                unsafe {
                    // Get current extended window styles
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    // Add layered, transparent, and no-activate styles
                    // - WS_EX_LAYERED: Required for transparency effects
                    // - WS_EX_TRANSPARENT: Makes window click-through
                    // - WS_EX_NOACTIVATE: Prevents window from stealing focus
                    SetWindowLongW(
                        hwnd,
                        GWL_EXSTYLE,
                        ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32 | WS_EX_NOACTIVATE.0 as i32
                    );
                }
            }

            let _tray = TrayIconBuilder::new()
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { button, button_state, .. } = event {
                        if button_state == MouseButtonState::Up {
                            let app = tray.app_handle();

                            match button {
                                MouseButton::Left => {
                                    if let Some(window) = app.get_webview_window("main") {
                                        if window.is_visible().unwrap_or(false) {
                                            let _ = window.hide();
                                        } else {
                                            let _ = window.show();
                                            let _ = window.set_ignore_cursor_events(true);

                                            #[cfg(target_os = "windows")]
                                            {
                                                use windows::Win32::Foundation::HWND;
                                                use windows::Win32::UI::WindowsAndMessaging::{
                                                    SetWindowPos, HWND_TOPMOST,
                                                    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE
                                                };

                                                if let Ok(hwnd) = window.hwnd() {
                                                    unsafe {
                                                        let _ = SetWindowPos(
                                                            HWND(hwnd.0),
                                                            HWND_TOPMOST,
                                                            0, 0, 0, 0,
                                                            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                MouseButton::Middle => {
                                    app.exit(0);
                                }
                                _ => {}
                            }
                        }
                    }
                })
                .build(app)?;

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["ctrl+alt+space"])?
                        .with_handler(|app, shortcut, event| {
                            // Only respond to key press events (not release)
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::CONTROL | Modifiers::ALT, Code::Space) {
                                    if let Some(window) = app.get_webview_window("main") {
                                        // Toggle window visibility
                                        if window.is_visible().unwrap_or(false) {
                                            let _ = window.hide();
                                        } else {
                                            let _ = window.show();
                                            // Re-apply click-through setting after showing the window.
                                            let _ = window.set_ignore_cursor_events(true);

                                            // Keep window on top without stealing focus
                                            #[cfg(target_os = "windows")]
                                            {
                                                use windows::Win32::Foundation::HWND;
                                                use windows::Win32::UI::WindowsAndMessaging::{
                                                    SetWindowPos, HWND_TOPMOST,
                                                    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE
                                                };

                                                if let Ok(hwnd) = window.hwnd() {
                                                    unsafe {
                                                        // Position window as topmost without changing size/position or activating it
                                                        // - HWND_TOPMOST: Places window above all non-topmost windows
                                                        // - SWP_NOMOVE | SWP_NOSIZE: Preserve current position and size
                                                        // - SWP_NOACTIVATE: Don't activate or focus the window
                                                        let _ = SetWindowPos(
                                                            HWND(hwnd.0),
                                                            HWND_TOPMOST,
                                                            0, 0, 0, 0,
                                                            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .build(),
                )?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
