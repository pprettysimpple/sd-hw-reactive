use simple_logger::SimpleLogger;
use tokio_postgres::Config;
use catlog::server::run_catlog_server;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let mut pg_config = Config::new();
    pg_config
        .host("localhost")
        .user("test")
        .password("test")
        .port(5555)
        .connect_timeout(std::time::Duration::from_secs(3));

    println!("Starting catlog server...");
    println!("Postgres config: {:?}", pg_config);

    run_catlog_server(pg_config).await;
}
