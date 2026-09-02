mod application;
mod commands;
mod domain;
mod storage;

use std::fs;

use application::AssetService;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_directory = app.path().app_data_dir().map_err(|error| {
                std::io::Error::other(format!(
                    "Could not resolve Peng app data directory: {error}"
                ))
            })?;
            fs::create_dir_all(&data_directory).map_err(|error| {
                std::io::Error::other(format!("Could not create Peng app data directory: {error}"))
            })?;
            let service =
                AssetService::open(&data_directory.join("peng.sqlite3")).map_err(|error| {
                    std::io::Error::other(format!("{}: {}", error.code, error.message))
                })?;
            app.manage(service);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::assets::create_asset,
            commands::assets::get_asset,
            commands::assets::update_asset,
            commands::assets::delete_asset
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Peng");
}
