mod structs;
mod utils;

use self::structs::{ActiveServer, Server, ServerInfo};
use self::utils::{format_rtt, generate_random_string};
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use reqwest::{Client, Result};

async fn get_active_server(client: &Client, uuid: &str) -> Result<ActiveServer> {
    let url = format!("https://{}.test.nextdns.io/", uuid);
    let resp = client.get(&url).send().await?;
    let active_server: ActiveServer = resp.json().await?;
    Ok(active_server)
}

async fn get_servers(client: &Client) -> Result<Vec<Server>> {
    let url = "https://router.nextdns.io/?source=ping";
    let resp = client.get(url).send().await?;
    let servers: Vec<Server> = resp.json().await?;
    Ok(servers)
}

async fn check_ipv6(client: &Client) -> bool {
    let url = "https://test-ipv6.nextdns.io/";
    let resp = client.get(url).send().await;
    match resp {
        Ok(r) => r.text().await.unwrap() == "OK",
        _ => false,
    }
}

async fn get_info(client: &Client, server: &Server, is_ipv6: bool) -> Option<(String, String)> {
    let protocol = if is_ipv6 { "ipv6" } else { "ipv4" };
    let url = format!(
        "https://{}-{}.edge.nextdns.io/info",
        protocol, server.server
    );
    let resp = client.get(&url).send().await;

    match resp {
        Ok(r) => {
            let rjson: ServerInfo = r.json().await.unwrap();
            Some((rjson.pop, format_rtt(rjson.rtt)))
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let rand_str = generate_random_string(20);
    let active_server = get_active_server(&client, &rand_str).await.unwrap();
    let servers = get_servers(&client).await.unwrap();
    let network_supports_ipv6 = check_ipv6(&client).await;

    let client_ip = if active_server.client.is_some() {
        IpAddr::from_str(&active_server.client.unwrap()).unwrap()
    } else {
        IpAddr::from_str("0.0.0.0").unwrap()
    };

    for server in servers {
        if server.ipv4 {
            let is_active = if server.server == active_server.server
                && active_server.status == "ok"
                && client_ip.is_ipv4()
            {
                "■"
            } else {
                " "
            };

            if let Some((pop, rtt)) = get_info(&client, &server, false).await {
                println!("{} {} {}", is_active, pop, rtt);
            } else {
                println!("{} {} error", is_active, server.pop);
            }
        }

        if server.ipv6 && network_supports_ipv6 {
            let is_active = if server.server == active_server.server
                && active_server.status == "ok"
                && client_ip.is_ipv6()
            {
                "■"
            } else {
                " "
            };

            if let Some((pop, rtt)) = get_info(&client, &server, true).await {
                println!("{} {} (IPv6) {}", is_active, pop, rtt);
            } else {
                println!("{} {} (IPv6) error", is_active, server.pop);
            }
        }
    }
}
