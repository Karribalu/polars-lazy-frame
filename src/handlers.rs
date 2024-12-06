use crate::db;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio_postgres::Client;
use tracing::{error, info};

pub(crate) async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn get_users(client: web::Data<Arc<Client>>) -> impl Responder {
    info!("Fetching users from database...");
    match db::fetch_users_as_lazyframe(&client).await {
        Ok((lazy_frame, users)) => {
            // Materialize LazyFrame into DataFrame
            info!("Fetched users successfully.");
            let dataframe = lazy_frame.collect().unwrap();
            info!("The data frame is {:?}", dataframe);
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            error!("Error fetching users: {}", e);
            HttpResponse::Ok().body(format!("Error: {}", e))
        }
    }
}

// Configure routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::get().to(get_users))
    ).route("/hey", web::get().to(hello));
}


