mod handlers;
mod models;
mod db;

use crate::handlers::config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Config;
use tracing::{error, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting the server...");
    dotenv().ok();

    // Load database configuration
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = Config::from_str(&database_url)
        .unwrap()
        .connect(connector)
        .await
        .expect("Failed to connect to database");

    info!("Connection to postgres is successful!");

    // Spawn a task to manage the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Connection error: {}", e);
        }
    });

    // Initialize the database
    db::initialize_db(&client).await.expect("Failed to initialize database");

    let client = Arc::new(client);
    let cloned = Arc::clone(&client);
    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&client))) // Share database client
            .configure(config)
    })
        .bind(("127.0.0.1", 8102))?
        .run()
        .await
}
