use crate::management_engine::controllers::auth::auth::{login_logic, register_logic};
use crate::management_engine::models::auth::auth::{AuthRequest, TokenResponse};
use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::{error, info};

#[post("/register")]
pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    req: web::Json<AuthRequest>,
) -> impl Responder {
    // Логируем входящий запрос
    info!("Вошли в /register с телом: {:?}", req);

    let result = register_logic(&pool, &req).await;

    // Логируем результат вызова логики
    info!("Результат register_logic: {:?}", result);

    match result {
        Ok(resp) => {
            info!("Регистрация пользователя {} успешна", req.username);
            HttpResponse::Ok().json(resp)
        }
        Err(msg) => {
            error!("Ошибка регистрации пользователя {}: {}", req.username, msg);
            HttpResponse::BadRequest().body(msg)
        }
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<sqlx::PgPool>, req: web::Json<AuthRequest>) -> impl Responder {
    // Логируем входящий запрос
    info!("Вошли в /login с пользователем: {}", req.username);

    let result = login_logic(&pool, &req).await;

    match result {
        Ok(resp) => {
            info!("Пользователь {} успешно вошел", req.username);
            HttpResponse::Ok().json(resp)
        }
        Err(msg) => {
            error!("Ошибка входа пользователя {}: {}", req.username, msg);
            HttpResponse::Unauthorized().body(msg)
        }
    }
}
