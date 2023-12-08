use crate::{save_theme_value, Theme};
use tauri::{command, AppHandle, Runtime};

#[command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    save_theme_value(&app.config(), theme);
    app.restart();
    Ok(())
}
