use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub currency: String,
}

impl User {
    pub fn new(id: String, currency: String) -> Self {
        Self {
            id,
            currency,
        }
    }
}
