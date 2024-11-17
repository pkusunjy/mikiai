use serde::{Deserialize, Serialize};
pub mod platform;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
