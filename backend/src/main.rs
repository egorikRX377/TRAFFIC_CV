use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tracing::{info, Level};
use tracing_subscriber;

mod management_engine;

use management_engine::api::auth::{login, register};
use management_engine::api::operator_api::{
    receive_telemetry,
    get_telemetry,
    start_generator_polling
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("=== BACKEND START ===");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("❌ Не удалось подключиться к базе");

    let pool = web::Data::new(pool);

    // ------------------------------------------------------------
    // 🔥 Запуск фонового процесса опроса генератора каждые 20 сек
    // ------------------------------------------------------------
    {
        let pool_clone = pool.clone();
        tokio::spawn(async move {
            start_generator_polling(pool_clone).await;
        });
    }

    info!("HTTP сервер => http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .app_data(pool.clone())
            .service(register)
            .service(login)
            .service(receive_telemetry)  // <-- POST вручную
            .service(get_telemetry)      // <-- GET для фронта
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
