use actix_web::{post, get, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use tracing::{error, info};
use reqwest;

#[derive(Debug, Deserialize, Clone)]
pub struct TelemetryEvent {
    pub device_name: String,
    pub ip_address: String,
    pub location: Option<String>,
    pub metric_type_id: i32,
    pub metric_value: f64,
    pub action_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TelemetryResponse {
    pub device_name: String,
    pub ip_address: String,
    pub location: Option<String>,
    pub metric_value: f64,
    pub is_anomaly: bool,
    pub action_description: Option<String>,
    pub recorded_at: Option<NaiveDateTime>,
}

// ------------------ Вставка события с проверкой аномалии ------------------
pub async fn insert_event_to_db(pool: &web::Data<PgPool>, event: &TelemetryEvent) -> Result<(), sqlx::Error> {
    info!("Начало вставки события: {:?}", event);

    // Вставка/получение устройства
    let device_id = match sqlx::query_scalar!(
        r#"
        INSERT INTO devices (device_name, ip_address, location)
        VALUES ($1, $2, $3)
        ON CONFLICT (device_name) DO UPDATE SET device_name = EXCLUDED.device_name
        RETURNING id
        "#,
        event.device_name,
        event.ip_address,
        event.location
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(id) => {
            info!("Устройство вставлено/найдено, device_id={}", id);
            id
        }
        Err(err) => {
            error!("Ошибка при вставке/поиске устройства: {:?}, событие={:?}", err, event);
            return Err(err);
        }
    };

    // Получаем critical_level для данного metric_type
    let threshold_row = match sqlx::query!(
        r#"SELECT critical_level FROM thresholds WHERE metric_type_id = $1"#,
        event.metric_type_id
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(row) => {
            info!("Порог для metric_type_id {} получен: {:?}", event.metric_type_id, row);
            row
        }
        Err(err) => {
            error!("Ошибка при получении порога для metric_type_id {}: {:?}, событие={:?}", event.metric_type_id, err, event);
            return Err(err);
        }
    };

    // Проверка на аномалию
    let is_anomaly = match threshold_row {
        Some(t) => {
            if let Some(critical) = t.critical_level {
                match critical.to_f64() {
                    Some(critical_f64) => {
                        let result = event.metric_value >= critical_f64;
                        info!("Проверка аномалии: metric_value={} >= critical_level={} -> {}", event.metric_value, critical_f64, result);
                        result
                    }
                    None => {
                        error!("Не удалось преобразовать BigDecimal {} в f64, событие={:?}", critical, event);
                        false
                    }
                }
            } else {
                info!("critical_level отсутствует для metric_type_id {}, событие={:?}", event.metric_type_id, event);
                false
            }
        }
        None => {
            info!("Порог не найден для metric_type_id {}, событие={:?}", event.metric_type_id, event);
            false
        }
    };

    // Вставка телеметрии
    let metric_value_bd = BigDecimal::from_f64(event.metric_value).unwrap_or_else(|| {
        error!("Ошибка преобразования metric_value {} в BigDecimal, событие={:?}", event.metric_value, event);
        BigDecimal::from(0)
    });

    match sqlx::query!(
        r#"
        INSERT INTO telemetry_data
            (device_id, metric_type_id, metric_value, is_anomaly, action_description, recorded_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        device_id,
        event.metric_type_id,
        metric_value_bd,
        is_anomaly,
        event.action_description,
        chrono::Utc::now().naive_utc()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => info!("Телеметрия успешно вставлена, событие={:?}, is_anomaly={}", event, is_anomaly),
        Err(err) => {
            error!("Ошибка при вставке телеметрии: {:?}, событие={:?}", err, event);
            return Err(err);
        }
    };

    Ok(())
}

// ==================== POST /operator/telemetry ====================
#[post("/operator/telemetry")]
pub async fn receive_telemetry(
    pool: web::Data<PgPool>,
    events: web::Json<Vec<TelemetryEvent>>,
) -> impl actix_web::Responder {
    info!("POST /operator/telemetry получено {} событий", events.len());
    for event in events.iter() {
        if let Err(err) = insert_event_to_db(&pool, event).await {
            error!("Ошибка вставки события: {:?}, событие={:?}", err, event);
        }
    }
    HttpResponse::Ok().body("Telemetry processed")
}

// ==================== GET /operator/telemetry ====================
#[get("/operator/telemetry")]
pub async fn get_telemetry(pool: web::Data<PgPool>) -> impl actix_web::Responder {
    info!("GET /operator/telemetry");

    let rows = match sqlx::query!(
        r#"
        SELECT 
            d.device_name, 
            d.ip_address, 
            d.location,
            t.metric_value::float8 as metric_value,
            t.is_anomaly,
            t.action_description,
            t.recorded_at
        FROM telemetry_data t
        JOIN devices d ON t.device_id = d.id
        ORDER BY t.recorded_at DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => rows,
        Err(err) => {
            error!("Ошибка при получении телеметрии: {:?}", err);
            return HttpResponse::InternalServerError().body("DB error");
        }
    };

    let response: Vec<TelemetryResponse> = rows
        .into_iter()
        .map(|r| TelemetryResponse {
            device_name: r.device_name,
            ip_address: r.ip_address,
            location: r.location,
            metric_value: r.metric_value.unwrap_or(0.0),
            is_anomaly: r.is_anomaly.unwrap_or(false),
            action_description: r.action_description,
            recorded_at: r.recorded_at,
        })
        .collect();

    info!("Возвращено {} записей", response.len());
    HttpResponse::Ok().json(response)
}

// ==================== POLLING ГЕНЕРАТОРА (каждые 20 сек) ====================
pub async fn start_generator_polling(pool: web::Data<PgPool>) {
    let client = reqwest::Client::new();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(20));

    loop {
        interval.tick().await;
        match client.get("http://127.0.0.1:10250/events").send().await {
            Ok(resp) => {
                match resp.json::<Vec<TelemetryEvent>>().await {
                    Ok(events) => {
                        info!("Получено {} событий от генератора", events.len());
                        for event in events.iter() {
                            if let Err(err) = insert_event_to_db(&pool, event).await {
                                error!("Ошибка при вставке события из генератора: {:?}, событие={:?}", err, event);
                            }
                        }
                    }
                    Err(err) => error!("Ошибка десериализации ответа генератора: {:?}", err),
                }
            }
            Err(err) => error!("Ошибка при запросе генератора: {:?}", err),
        }
    }
}
