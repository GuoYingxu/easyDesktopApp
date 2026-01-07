// use tauri::{App, tray::TrayIconBuilder};
use tauri::{
  App,Manager, menu::{Menu, MenuItem}, tray::{ MouseButton, TrayIconBuilder, TrayIconEvent}
};

/*
创建托盘
*/

pub fn create_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
  let quit_i = MenuItem::with_id(app,"quit","Quit",true, None::<&str>)?;
  let menu = Menu::with_items(app,&[&quit_i])?;
  let _tray = TrayIconBuilder::new()
  .icon(app.default_window_icon().unwrap().clone())
  .menu(&menu)
  .show_menu_on_left_click(false)
  .on_menu_event(|app,event| match event.id.as_ref(){
    "quit" =>{
      app.exit(0);
    }
    _ => {
      // println!("menu item {:?} clicked", event.id);
    }
  })
  .on_tray_icon_event(|tray,event| match event {
    TrayIconEvent::Click { button, ..} => match button  {
      MouseButton::Left => {
        // println!("left click");
        // left click: restore/unminimize main window
        let app = tray.app_handle();
        if let Some(window) = app.get_webview_window("main") {
          let _ = window.unminimize();
          let _ = window.show();
          let _ = window.set_focus();
        }
      },
      MouseButton::Right => {},
      _ => {},
        // println!("middle click");
    }
    _ => {
      // other events (including right-click) are handled by the system/menu
    }
  })
  .build(app)?;
  Ok(())
}