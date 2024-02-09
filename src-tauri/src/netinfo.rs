use sysinfo::Networks;

#[tauri::command]
pub fn getnetinfo() -> Vec<String> {
    let networks = Networks::new_with_refreshed_list();
    let mut interfacelist = Vec::new();
    for (interface_name, _) in &networks {
        interfacelist.push(interface_name.to_string());
    }
    interfacelist
}
