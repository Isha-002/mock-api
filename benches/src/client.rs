use futures::stream::{self, StreamExt};
use rand::Rng;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;
use tokio;
use tokio::sync::Mutex;

pub struct RestaurantApiClient {
    client: Client,
    base_url: String,
    id_counter: Arc<Mutex<u64>>,
}

impl RestaurantApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            id_counter: Arc::new(Mutex::new(1)),
        }
    }

    pub async fn send_requests(&self, num_requests: usize, concurrency: usize) {
        let start_time = Instant::now();
        let requests = stream::iter(0..num_requests).map(|_| {
            let client = self.client.clone();
            let url = format!("{}/restaurants", self.base_url);
            let id_counter = self.id_counter.clone();

            tokio::spawn(async move {
                let id = Self::generate_unique_id(&id_counter).await;
                let data = Self::generate_random_restaurant(id);

                let request_start = Instant::now();

                let response = client.post(&url).json(&data).send().await;

                let request_duration = request_start.elapsed();
                match response {
                    Ok(resp) => {
                        println!(
                            "Response: {:?} - Time taken: {:?}",
                            resp.status(),
                            request_duration
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {:?} - Time taken: {:?}", e, request_duration);
                    }
                }
            })
        });

        requests
            .buffer_unordered(concurrency)
            .for_each(|_| async {})
            .await;

        let total_duration = start_time.elapsed();
        let avg_req_per_sec = num_requests as f64 / total_duration.as_secs_f64();
        println!(
            "All requests completed in {:?}. Average requests per second: {:.2}",
            total_duration, avg_req_per_sec
        );
    }

    async fn generate_unique_id(id_counter: &Arc<Mutex<u64>>) -> String {
        let mut counter = id_counter.lock().await;
        let id = *counter;
        *counter += 1;
        id.to_string()
    }

    fn generate_random_restaurant(id: String) -> serde_json::Value {
        let mut rng = rand::thread_rng();
        let rating: f64 = rng.gen_range(1.0..=5.0);
        let distance: f64 = rng.gen_range(1.0..=10.0);
        let names = [
            "Bodega",
            "Maggie",
            "Akbar Joojeh",
            "Pasta House",
            "Burger Town",
        ];
        let tags = vec![
            vec!["Tacos", "Burritos"],
            vec!["Pizza", "Pasta"],
            vec!["Sushi", "Tempura"],
        ];
        let images = [
            "https://example.com/bodega.jpg",
            "https://example.com/maggie.jpg",
            "https://example.com/akbar.jpg",
        ];

        let name = names[rng.gen_range(0..names.len())];
        let tag_list = tags[rng.gen_range(0..tags.len())].clone();
        let image = images[rng.gen_range(0..images.len())];

        json!({
            "id": id,
            "name": name,
            "rating": rating,
            "distance": distance,
            "tags": tag_list,
            "image": image
        })
    }
}
