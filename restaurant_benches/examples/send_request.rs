use restaurant_benches::client::RestaurantApiClient;

#[tokio::main]
async fn main() {
    let api_client = RestaurantApiClient::new("http://localhost:4444");
    api_client.send_requests(100000, 10).await;
}
