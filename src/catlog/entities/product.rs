use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub currency: String,
}

impl Product {
    pub fn new(id: String, currency: String) -> Self {
        Self {
            id,
            currency,
        }
    }
}
