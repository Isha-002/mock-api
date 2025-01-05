use benches::client::RestaurantApiClient;

#[tokio::main]
async fn main() {
    let api_client = RestaurantApiClient::new("http://localhost:4444");
    api_client.send_requests(100, 10).await;
}
