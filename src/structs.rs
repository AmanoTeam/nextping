use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub pop: String,
    pub server: String,
    pub ipv4: bool,
    pub ipv6: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub location_name: String,
    pub pop: String,
    pub rtt: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveServer {
    pub status: String,
    pub protocol: String,
    pub profile: String,
    pub client: String,
    pub anycast: bool,
    pub server: String,
    pub client_name: String,
    pub device_name: String,
}
