use tauri::command;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::Name;
use std::str::FromStr;

#[command]
pub fn check_dns_pollution(domain: String) -> Result<bool, String> {
    let public_resolver = Resolver::new(ResolverConfig::google(), ResolverOpts::default())
        .map_err(|e| e.to_string())?;
    let system_resolver = Resolver::from_system_conf()
        .map_err(|e| e.to_string())?;

    let name = Name::from_str(&domain).map_err(|e| e.to_string())?;
    let public_response = public_resolver.lookup_ip(name.clone()).map_err(|e| e.to_string())?;
    let system_response = system_resolver.lookup_ip(name).map_err(|e| e.to_string())?;

    // Check if all system resolver IPs are not in the public resolver IP list
    let all_system_ips_not_in_public = system_response.iter().all(|system_ip| {
        !public_response.iter().any(|public_ip| public_ip == system_ip)
    });

    Ok(all_system_ips_not_in_public)
}