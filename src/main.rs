mod structs;
mod utils;

use structs::{ActiveServer, Server};
use utils::{format_rtt, generate_random_string};

use reqwest::Client;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_http_client();
    let rand_str = generate_random_string(20);
    let active_server = get_active_server(&client, &rand_str).await?;
    let servers = get_servers(&client).await?;
    let network_supports_ipv6 = check_ipv6(&client).await;

    let client_ip = get_client_ip(&active_server)?;

    for server in servers {
        if server.ipv4 {
            print_server_info(&client, &server, false, &active_server, &client_ip).await;
        }

        if server.ipv6 && network_supports_ipv6 {
            print_server_info(&client, &server, true, &active_server, &client_ip).await;
        }
    }

    Ok(())
}

fn create_http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to create HTTP client")
}

async fn get_active_server(client: &Client, uuid: &str) -> Result<ActiveServer, reqwest::Error> {
    let url = format!("https://{}.test.nextdns.io/", uuid);
    let resp = client.get(&url).send().await?;
    let active_server: ActiveServer = resp.json().await?;
    Ok(active_server)
}

async fn get_servers(client: &Client) -> Result<Vec<Server>, reqwest::Error> {
    let url = "https://router.nextdns.io/?source=ping";
    let resp = client.get(url).send().await?;
    let servers: Vec<Server> = resp.json().await?;
    Ok(servers)
}

async fn check_ipv6(client: &Client) -> bool {
    let url = "https://[2606:4700:4700::1111]/cdn-cgi/trace";
    let resp = client.get(url).send().await;
    resp.is_ok()
}

fn get_client_ip(active_server: &ActiveServer) -> Result<IpAddr, std::net::AddrParseError> {
    match &active_server.client {
        Some(client_str) => IpAddr::from_str(client_str),
        None => Ok(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
    }
}

async fn print_server_info(
    client: &Client,
    server: &Server,
    is_ipv6: bool,
    active_server: &ActiveServer,
    client_ip: &IpAddr,
) {
    let protocol = if is_ipv6 { "ipv6" } else { "ipv4" };
    let url = format!(
        "https://{}-{}.edge.nextdns.io/info",
        protocol, server.server
    );
    if let Ok(resp) = client.get(&url).send().await {
        match resp.json::<structs::ServerInfo>().await {
            Ok(info) => {
                let pop = &info.pop;
                let rtt = format_rtt(info.rtt);
                let is_active = if server.server == active_server.server
                    && active_server.status == "ok"
                    && client_ip.is_ipv6() == is_ipv6
                {
                    "■"
                } else {
                    " "
                };
                let ipv6_str = if is_ipv6 { " (IPv6)" } else { "" };
                println!("{} {}{} {}", is_active, pop, ipv6_str, rtt);
            }
            _ => println!("Error decoding JSON for {}", server.pop),
        }
    } else {
        println!("Error accessing server info for {}", server.pop);
    }
}
