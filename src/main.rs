use reqwest::Client;
use uuid::Uuid;

fn format_rtt(rtt: f64) -> String {
    let rtt = rtt / 1000.0;
    format!("{:.1} ms", rtt)
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let request1 = client.get("https://router.nextdns.io/?source=ping").send();
    let request2 = client.get("https://test-ipv6.nextdns.io/").send();
    let uuid = Uuid::new_v4().to_string();
    let active_router = client
        .get(format!("https://{}.test.nextdns.io/", uuid))
        .send();
    let active_server = active_router
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let (response1, response2) = tokio::join!(request1, request2);

    let response1 = response1.unwrap();
    let response2 = response2.unwrap();

    let network_supports_ipv6 = response2.text().await.unwrap() == "OK";

    for server in response1
        .json::<serde_json::Value>()
        .await
        .unwrap()
        .as_array()
        .unwrap()
    {
        let server = server.as_object().unwrap();
        let server_name = server.get("server").unwrap().as_str().unwrap();
        let ipv4 = server.get("ipv4").unwrap().as_bool().unwrap();
        let ipv6 = server.get("ipv6").unwrap().as_bool().unwrap();

        let is_active = if server_name == active_server["server"].as_str().unwrap() {
            "■"
        } else {
            " "
        };

        if ipv4 {
            let request = client
                .get(format!("https://ipv4-{}.edge.nextdns.io/info", server_name))
                .send();
            let response = request.await.unwrap();
            let rjson = response.json::<serde_json::Value>().await.unwrap();
            let pop = rjson["pop"].as_str().unwrap();
            let rtt = rjson["rtt"].as_f64().unwrap();
            println!("{} {} {}", is_active, pop, format_rtt(rtt));
        }

        if ipv6 && network_supports_ipv6 {
            let request = client
                .get(format!("https://ipv6-{}.edge.nextdns.io/info", server_name))
                .send();
            let response = request.await.unwrap();
            let rjson = response.json::<serde_json::Value>().await.unwrap();
            let pop = rjson["pop"].as_str().unwrap();
            let rtt = rjson["rtt"].as_f64().unwrap();
            println!("{} {} (IPv6) {}", is_active, pop, format_rtt(rtt));
        }
    }
}
