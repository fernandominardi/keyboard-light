use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Make window click-through
            #[cfg(target_os = "windows")]
            {
                use windows::Win32::Foundation::HWND;
                use windows::Win32::UI::WindowsAndMessaging::{
                    GetWindowLongW, SetWindowLongW, GWL_EXSTYLE,
                    WS_EX_LAYERED, WS_EX_TRANSPARENT, WS_EX_NOACTIVATE
                };

                let hwnd = HWND(window.hwnd().unwrap().0);
                unsafe {
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    SetWindowLongW(
                        hwnd,
                        GWL_EXSTYLE,
                        ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32 | WS_EX_NOACTIVATE.0 as i32
                    );
                }
            }

            // Register global shortcut using the recommended Builder pattern
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["ctrl+space"])?
                        .with_handler(|app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::CONTROL, Code::Space) {
                                    if let Some(window) = app.get_webview_window("main") {
                                        if window.is_visible().unwrap_or(false) {
                                            let _ = window.hide();
                                        } else {
                                            let _ = window.show();

                                            // Prevent focus stealing on Windows
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
