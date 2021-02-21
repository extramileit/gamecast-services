use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Venue {
    venue_id: i32,
    venue_name: String,
}
