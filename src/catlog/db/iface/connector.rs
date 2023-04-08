use crate::entities::product::Product;
use crate::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait Connector {
    async fn create_user(&self, user: User) -> Result<(), String>;
    async fn get_user(&self, id: String) -> Result<User, String>;
    async fn add_product(&self, product: Product) -> Result<(), String>;
    async fn get_product(&self, id: String) -> Result<Product, String>;
    async fn get_available_products(&self, user_id: String) -> Result<Vec<Product>, String>;
    async fn get_products_by_currency(&self, currency: String) -> Result<Vec<Product>, String>;
}
