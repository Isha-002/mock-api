struct Restaurant {
    id: RestaurantId,
    name: String,
    rating: f32,
    distance: f64,
    tags: Option<Vec<String>>,
    image: String,
}

struct RestaurantId(String);

impl Restaurant {
    fn new(
        id: RestaurantId,
        name: &str,
        rating: f32,
        distance: f64,
        tags: Option<Vec<&str>>,
        image: &str,
    ) -> Self {
        Restaurant {
            id,
            name: name.to_string(),
            rating,
            distance,
            tags: tags.map(|t| t.into_iter().map(|s| s.to_string()).collect()),
            image: image.to_string(),
        }
    }
    fn update_name(&self, new_name: &str) -> Self {
        Restaurant::new(self.id, new_name, self.rating, self.distance, self.tags, &self.image)
    }
}



fn main() {
    let res = Restaurant::new(
        RestaurantId("1".to_string()),
        "kebab".to_string(),
        3.5,
        2.5,
        Some(vec!["aaa".to_string(), "aaa".to_string()]),
        "img".to_string(),
    );
    println!("Hello, world!");
}
