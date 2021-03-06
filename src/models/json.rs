use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredDevice {
    pub name: String,
    pub dns_domain: String,
    pub snmp_community: Option<String>,
    pub base_mac: Option<String>,
    pub os_info: Option<String>,
    pub interfaces : HashMap<String, DiscoveredInterface>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredInterface {
    pub index: i32,
    pub interface_type: String,
    pub display_name: Option<String>,
    pub name: String,
    pub alias: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkPeerInfo {
    pub name : String,
    pub dns_domain : String,
    pub interface : String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkInfo {
    pub device_fqdn : String,
    pub interfaces : HashMap<String, Option<LinkPeerInfo>>,
    pub topology_stable : bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceMonitorInfo {
    pub fqdn : String,
    pub up : Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceMonitorResponse {
    pub devices : Vec<DeviceMonitorInfo>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceMonitorReport {
    pub fqdn : String,
    pub up : bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InterfaceMonitorInterfaceReport {
    pub if_index: i32,
    pub in_octets: Option<u64>,
    pub out_octets: Option<u64>,
    pub in_packets: Option<u64>,
    pub out_packets: Option<u64>,
    pub in_errors: Option<u64>,
    pub out_errors: Option<u64>,
    pub up: Option<bool>,
    pub speed: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InterfaceMonitorReport {
    pub device_fqdn : String,
    pub interfaces: Vec<InterfaceMonitorInterfaceReport>,
}