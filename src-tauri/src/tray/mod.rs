//! System tray implementation

use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    App, Manager,
};

/// Setup system tray
pub fn setup_tray(app: &App) -> anyhow::Result<()> {
    // Create menu items
    let show_item = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // Build menu
    let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

    // Build tray icon
    let _tray = TrayIconBuilder::new()
        .icon(Image::from_bytes(include_bytes!("../icons/icon.png"))?)
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.show().ok();
                    window.set_focus().ok();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.hide().ok();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
