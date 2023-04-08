use tokio_postgres::{Config, Row};
use tokio_postgres::types::{ToSql};

use async_trait::async_trait;

use crate::db::iface::connector::Connector;
use crate::entities::product::Product;
use crate::entities::user::User;

// connects to postgres with tokio_postgres
// runs queries and parses results
#[derive(Clone)]
pub struct ServiceContext {
    config: Config,
}

impl ServiceContext {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run_query_async(&self, query: &str, params: Vec<&str>) -> Result<Vec<Row>, String> {
        let (client, connection) = self.config.connect(tokio_postgres::NoTls).await
            .map_err(|e| format!("Error connecting to database: {}", e))?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let params: Vec<&(dyn ToSql + Sync)> = params
            .iter()
            .map(|p| p as &(dyn ToSql + Sync))
            .collect();

        client
            .query(query, params.as_slice()).await
            .map_err(|e| format!("Error executing query: {}", e))
    }

    pub async fn parse_user(&self, row: &Row) -> Result<User, String> {
        let id = row.get(0);
        let currency = row.get(1);

        Ok(User::new(id, currency))
    }

    pub async fn parse_product(&self, row: &Row) -> Result<Product, String> {
        let id = row.get(0);
        let currency = row.get(1);

        Ok(Product::new(id, currency))
    }
}

#[async_trait]
impl Connector for ServiceContext {
    async fn create_user(&self, user: User) -> Result<(), String> {
        let query = "INSERT INTO users (id, currency) VALUES ($1, $2)";

        let params = vec![
            user.id.as_str(),
            user.currency.as_str()
        ].into();

        self.run_query_async(query, params).await.map(|_| ())
    }

    async fn get_user(&self, id: String) -> Result<User, String> {
        let query = "SELECT * FROM users WHERE id = $1";

        let params = vec![
            id.as_str()
        ].into();

        let rows = self.run_query_async(query, params).await?;

        if rows.len() == 0 {
            return Err(format!("User with id {} not found", id));
        }

        self.parse_user(&rows[0]).await
    }

    async fn add_product(&self, product: Product) -> Result<(), String> {
        let query = "INSERT INTO products (id, currency) VALUES ($1, $2)";

        let params = vec![
            product.id.as_str(),
            product.currency.as_str()
        ].into();

        self.run_query_async(query, params).await.map(|_| ())
    }

    async fn get_product(&self, id: String) -> Result<Product, String> {
        let query = "SELECT * FROM products WHERE id = $1";

        let params = vec![
            id.as_str()
        ].into();

        let rows = self.run_query_async(query, params).await?;

        if rows.len() == 0 {
            return Err(format!("Product with id {} not found", id));
        }

        self.parse_product(&rows[0]).await
    }

    async fn get_available_products(&self, user_id: String) -> Result<Vec<Product>, String> {
        let query = "SELECT * FROM products WHERE currency = (SELECT currency FROM users WHERE id = $1)";

        let params = vec![
            user_id.as_str()
        ].into();

        let rows = self.run_query_async(query, params).await?;

        let mut products = Vec::new();

        for row in rows {
            products.push(self.parse_product(&row).await?);
        }

        Ok(products)
    }

    async fn get_products_by_currency(&self, currency: String) -> Result<Vec<Product>, String> {
        let query = "SELECT * FROM products WHERE currency = $1";

        let params = vec![
            currency.as_str()
        ].into();

        let rows = self.run_query_async(query, params).await?;

        let mut products = Vec::new();

        for row in rows {
            products.push(self.parse_product(&row).await?);
        }

        Ok(products)
    }
}
