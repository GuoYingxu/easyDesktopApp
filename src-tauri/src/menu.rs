use tauri::{
  App, menu::{MenuBuilder,  SubmenuBuilder}
};

pub fn create_menu(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
  let system_setting_menu = SubmenuBuilder::new(app, "设置" )
  .text("系统设置","Setting")
  .build()?;

  let menu = MenuBuilder::new(app).items(&[
    &system_setting_menu,
  ]).build()?;
  app.set_menu(menu)?;
  Ok(())
}