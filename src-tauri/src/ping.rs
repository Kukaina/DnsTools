// ping.rs

use surge_ping::ping;
use std::net::IpAddr;
use tauri::command;

#[command]
pub async fn pings(address: String) -> Result<i64, String> {
    // 解析地址为 IpAddr
    let address = address.parse::<IpAddr>().map_err(|_| "Invalid IP address".to_string())?;
    // 使用 surge_ping crate 提供的 ping 函数
    let result = ping(address, &[0; 32]).await.map_err(|e| e.to_string())?;
    // 将 Duration 转换为毫秒
    let duration_ms = result.1.as_secs_f64() * 1000.0;
    Ok(duration_ms as i64)
}