// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod ping;
mod getjsonlist;
mod netinfo;
mod hijack;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping::pings,getjsonlist::get_records,netinfo::getnetinfo,hijack::check_dns_pollution])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
