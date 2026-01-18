// Sellify Backend - 11 Deterministic Engines
pub mod engines;
pub mod models;
pub mod utils;
pub mod api;

// Re-export main engines
pub use engines::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // Initialize Sellify engines
      log::info!("Initializing Sellify Backend Engines...");
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
