use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct Status {
    url: String,
    name: String,
    is_up: bool,
    response_time: f32,
}

fn main() {
    println!("Hello, world!");
}
