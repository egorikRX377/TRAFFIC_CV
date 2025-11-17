use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};


#[derive(Serialize, Clone)]
struct TelemetryEvent {
    device_name: String,
    ip_address: String,
    location: String,
    metric_value: f64,
    action_description: String,
}
// =========================================================
// ENUMS
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "device_status", rename_all = "lowercase")]
pub enum DeviceStatus {
    Active,
    Warning,
    Inactive,
}

// =========================================================
// USERS
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role_id: Option<i32>,
    pub user_info_id: Option<i32>,
    pub created_at: NaiveDateTime,
}

// =========================================================
// USER INFO
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserInfo {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub organization: Option<String>,
}

// =========================================================
// ROLES
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Role {
    pub id: i32,
    pub role_name: String,
}

// =========================================================
// DEVICES
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Device {
    pub id: i32,
    pub device_name: String,
    pub ip_address: String,
    pub location: Option<String>,
    pub status: DeviceStatus,
    pub added_by: Option<i32>,      // FK → users.id
}

// =========================================================
// TELEMETRY DATA
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct TelemetryData {
    pub id: i32,
    pub device_id: Option<i32>,           // FK → devices.id
    pub metric_type_id: Option<i32>,      // FK → metric_types.id
    pub metric_value: f64,                // numeric(12,2)
    pub recorded_at: chrono::NaiveDateTime,
    pub is_anomaly: bool,                 // DEFAULT false
    pub action_description: Option<String>,
}

// =========================================================
// METRIC TYPES
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MetricType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

// =========================================================
// THRESHOLDS
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Threshold {
    pub id: i32,
    pub metric_type_id: Option<i32>, // FK → metric_types.id
    pub warning_level: Option<f64>,
    pub critical_level: Option<f64>,
    pub created_by: Option<i32>,     // FK → users.id
}

// =========================================================
// LOGS
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LogEntry {
    pub id: i32,
    pub user_id: Option<i32>,    // FK → users.id
    pub action: String,
    pub details: Option<String>,
    pub logged_at: NaiveDateTime,
}

// =========================================================
// MAILBOXES
// =========================================================

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Mailbox {
    pub id: i32,
    pub user_id: Option<i32>,         // FK → users.id
    pub title: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub read_status: bool,
}
