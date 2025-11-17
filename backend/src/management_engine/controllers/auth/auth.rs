use crate::management_engine::clients::traits::auth::AuthClient;
use crate::management_engine::models::auth::auth::{AuthRequest, Claims, TokenResponse};
use crate::management_engine::clients::clients::auth::auth::PgAuthClient;
use actix_web::web;
use bcrypt::{hash, verify};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use tracing::{error, info};

const DEFAULT_ROLE: &str = "оператор";


pub const SECRET_KEY: &str = "your_super_secret_key_change_me";

pub async fn register_logic(
    pool: &web::Data<sqlx::PgPool>,
    req: &AuthRequest,
) -> Result<TokenResponse, String> {
    let client = PgAuthClient {
        pool: pool.get_ref().clone(),
    };

    info!("Регистрация пользователя: {}", req.username);

    // Проверка существующего пользователя
    match client.get_user_details(&req.username).await {
        Ok(Some(_)) => {
            info!("Пользователь {} уже существует", req.username);
            return Err("Пользователь уже существует".to_string());
        }
        Err(e) => {
            error!("Ошибка проверки существующего пользователя: {:?}", e);
            return Err("Ошибка базы данных".to_string());
        }
        _ => {}
    }

    // Получение роли
    let role_id = match client.get_role_id(DEFAULT_ROLE).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            error!("Роль '{}' не найдена", DEFAULT_ROLE);
            return Err("Роль не найдена".to_string());
        }
        Err(e) => {
            error!("Ошибка получения роли: {:?}", e);
            return Err("Ошибка базы данных".to_string());
        }
    };

    // Проверка обязательных полей user_info
    let full_name = req.full_name.as_ref().ok_or("Поле full_name обязательно")?;
    let email = req.email.as_ref().ok_or("Поле email обязательно")?;

    info!("Создание user_info для {} / {}", full_name, email);
    let user_info_id = match client
        .create_user_info(
            full_name,
            email,
            req.phone_number.as_deref(),
            req.organization.as_deref(),
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            error!("Ошибка создания user_info: {:?}", e);
            return Err("Ошибка создания информации о пользователе".to_string());
        }
    };

    // Хэширование пароля
    let hashed = match hash(&req.password, 12) {
        Ok(h) => h,
        Err(e) => {
            error!("Ошибка хэширования пароля: {:?}", e);
            return Err("Ошибка хэширования пароля".to_string());
        }
    };

    // Создание пользователя
    info!("Создание пользователя {}", req.username);
    if let Err(e) = client
        .create_user(&req.username, &hashed, role_id, user_info_id)
        .await
    {
        error!("Ошибка создания пользователя: {:?}", e);
        return Err("Ошибка создания пользователя".to_string());
    }

    let token =
        generate_token(&req.username, DEFAULT_ROLE).map_err(|_| "Ошибка генерации токена")?;

    info!("Пользователь {} успешно зарегистрирован", req.username);
    Ok(TokenResponse { token })
}

pub async fn login_logic(
    pool: &web::Data<sqlx::PgPool>,
    req: &AuthRequest,
) -> Result<TokenResponse, String> {
    let client = PgAuthClient {
        pool: pool.get_ref().clone(),
    };

    info!("Попытка входа пользователя: {}", req.username);

    let user_opt: Option<(String, String)> = match client.get_user_details(&req.username).await {
        Ok(opt) => opt,
        Err(e) => {
            error!("Ошибка обращения к базе данных: {:?}", e);
            return Err("Ошибка обращения к базе данных".to_string());
        }
    };

    if let Some((hashed, role)) = user_opt {
        match verify(&req.password, &hashed) {
            Ok(true) => {
                let token =
                    generate_token(&req.username, &role).map_err(|_| "Ошибка генерации токена")?;
                info!("Пользователь {} успешно вошел", req.username);
                return Ok(TokenResponse { token });
            }
            Ok(false) => {
                info!("Неверный пароль для пользователя {}", req.username);
            }
            Err(e) => {
                error!("Ошибка проверки пароля: {:?}", e);
                return Err("Ошибка проверки пароля".to_string());
            }
        }
    } else {
        info!("Пользователь {} не найден", req.username);
    }

    Err("Неверные учетные данные".to_string())
}

fn generate_token(username: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        role: role.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
}
