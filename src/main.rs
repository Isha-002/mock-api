struct Restaurant {
    id: RestaurantId,
    name: String,
    rating: u8,
    distance: f64,
    tags: Option<Vec<String>>,
    image: String,
}

struct RestaurantId(String);

impl Restaurant {
    fn new(
        id: RestaurantId,
        name: String,
        rating: u8,
        distance: f64,
        tags: Option<Vec<String>>,
        image: String,
    ) -> Self {
        Restaurant {
            id,
            name,
            rating,
            distance,
            tags,
            image,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
