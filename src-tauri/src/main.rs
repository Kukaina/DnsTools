// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod getjsonlist;
mod ping;
mod pollute;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ping::pings,
            getjsonlist::get_records,
            pollute::check_dns_pollution,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
