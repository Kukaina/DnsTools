// // 使用DNS查询会导致缓存问题
// use serde::{Serialize, Deserialize};
// use std::time::Instant;
// use trust_dns_client::client::{Client, SyncClient};
// use trust_dns_client::udp::UdpClientConnection;
// use trust_dns_client::rr::{DNSClass, Name, RecordType};
// use std::net::SocketAddr;
// use tauri::command;

// // 结果结构体，用于序列化 JSON 响应
// #[derive(Serialize, Deserialize)]
// struct DnsQueryResult {
//     server: String,
//     query_time_ms: u128, // 查询时间，以毫秒为单位
//     success: bool,
//     error_message: Option<String>,
// }

// #[command]
// pub fn check_dns_latency(dnsip: String,domain:String) -> Result<String, String> {
//     let server = format!("{}:53", dnsip);
//     let socket_addr: SocketAddr = match server.parse() {
//         Ok(addr) => addr,
//         Err(e) => return Err(e.to_string()),
//     };

//     let conn = match UdpClientConnection::new(socket_addr) {
//         Ok(conn) => conn,
//         Err(e) => return Err(e.to_string()),
//     };
    
//     let client = SyncClient::new(conn);
//     let name = match Name::from_ascii(domain) {
//         Ok(name) => name,
//         Err(e) => return Err(e.to_string()),
//     };

//     let start_time = Instant::now();
//     let query_result = client.query(&name, DNSClass::IN, RecordType::A);
//     let duration = start_time.elapsed();

//     let result = match query_result {
//         Ok(_) => DnsQueryResult {
//             server: dnsip,
//             query_time_ms: duration.as_millis(),
//             success: true,
//             error_message: None,
//         },
//         Err(e) => DnsQueryResult {
//             server: dnsip,
//             query_time_ms: duration.as_millis(),
//             success: false,
//             error_message: Some(e.to_string()),
//         },
//     };

//     // 序列化结果为 JSON 字符串
//     match serde_json::to_string(&result) {
//         Ok(json_result) => Ok(json_result),
//         Err(e) => Err(e.to_string()),
//     }
// }