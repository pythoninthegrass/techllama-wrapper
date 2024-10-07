// TODO: fix imports and mutability
use tauri::{CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Window};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle();
      let mut shortcut_manager = handle.global_shortcut_manager();

      // Register the global shortcut
      shortcut_manager.register("Cmd+Shift+G", move || {
        let window = handle.get_webview_window("main").unwrap();
        if window.is_visible().unwrap() {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
        }
      }).unwrap();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
