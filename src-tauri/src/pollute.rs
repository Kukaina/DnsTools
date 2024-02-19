use tauri::command;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::lookup_ip::LookupIp;
use trust_dns_resolver::Name;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::net::IpAddr;
use tauri::InvokeError;

#[derive(Serialize, Deserialize)]
pub struct DnsCheckResult {
    all_system_ips_not_in_public: bool,
    public_response: Vec<IpAddr>,
    system_response: Vec<IpAddr>,
}

type Result<T> = std::result::Result<T, InvokeError>;

#[command]
pub fn check_dns_pollution(domain: String) -> Result<DnsCheckResult> {
    let public_resolver = Resolver::new(ResolverConfig::google(), ResolverOpts::default())
        .map_err(|e| InvokeError::from(e.to_string()))?;
    let system_resolver = Resolver::from_system_conf()
        .map_err(|e| InvokeError::from(e.to_string()))?;

    let name = Name::from_str(&domain).map_err(|e| InvokeError::from(e.to_string()))?;
    
    let public_response: LookupIp = public_resolver.lookup_ip(name.clone()).map_err(|e| InvokeError::from(e.to_string()))?;
    let system_response: LookupIp = system_resolver.lookup_ip(name).map_err(|e| InvokeError::from(e.to_string()))?;

    // 将LookupIp转换为Vec<IpAddr>
    let public_ips: Vec<IpAddr> = public_response.iter().map(|r| r).collect();
    let system_ips: Vec<IpAddr> = system_response.iter().map(|r| r).collect();

    // 检查所有系统解析器的IP是否都不在公共解析器的IP列表中
    let all_system_ips_not_in_public = system_ips.iter().all(|system_ip| {
        !public_ips.iter().any(|public_ip| public_ip == system_ip)
    });

    Ok(DnsCheckResult {
        all_system_ips_not_in_public,
        public_response: public_ips,
        system_response: system_ips,
    })
}