use serde::{Deserialize};

#[derive(Deserialize)]
pub struct IncrementPayload {
    pub count: u64
}