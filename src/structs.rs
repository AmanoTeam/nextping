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
#[serde(rename_all = "lowercase")]
pub struct ActiveServer {
    pub status: String,
    pub resolver: Option<String>,
    pub protocol: Option<String>,
    pub profile: Option<String>,
    pub client: Option<String>,
    pub srcip: Option<String>,
    pub anycast: Option<bool>,
    pub server: String,
    pub clientname: Option<String>,
    pub devicename: Option<String>,
    pub deviceid: Option<String>,
}
