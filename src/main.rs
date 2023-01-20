use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    pop: String,
    server: String,
    ipv4: bool,
    ipv6: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerInfo {
    location_name: String,
    pop: String,
    rtt: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ActiveServer {
    status: String,
    protocol: String,
    profile: String,
    client: String,
    anycast: bool,
    server: String,
    client_name: String,
    device_name: String,
}

fn format_rtt(rtt: f64) -> String {
    let rtt = rtt / 1000.0;
    format!("{:.1} ms", rtt)
}

fn generate_random_string(length: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    rand_string
}

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
    let client = Client::new();

    let rand_str = generate_random_string(20);
    let active_server = get_active_server(&client, &rand_str).await.unwrap();
    let servers = get_servers(&client).await.unwrap();
    let network_supports_ipv6 = check_ipv6(&client).await;

    for server in servers {
        let is_active = if server.server == active_server.server && active_server.status == "ok" {
            "■"
        } else {
            " "
        };

        if server.ipv4 {
            if let Some((pop, rtt)) = get_info(&client, &server, false).await {
                println!("{} {} {}", is_active, pop, rtt);
            }
        }

        if server.ipv6 && network_supports_ipv6 {
            if let Some((pop, rtt)) = get_info(&client, &server, true).await {
                println!("{} {} (IPv6) {}", is_active, pop, rtt);
            }
        }
    }
}
