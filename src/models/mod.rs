use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
