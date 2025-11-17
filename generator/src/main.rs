use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

#[derive(Serialize, Clone)]
struct TelemetryEvent {
    device_name: String,
    ip_address: String,
    location: String,
    metric_name: String,       
    metric_value: f64,
    action_description: String,
}

type SharedEvents = Arc<Mutex<Vec<TelemetryEvent>>>;

fn generate_event() -> TelemetryEvent {
    let devices = vec![
        "Router-01", "Router-05", "Router-22",
        "Switch-02", "Switch-10", "Switch-50",
        "Firewall-03", "Firewall-09", "Firewall-15",
    ];

    let ips = vec![
        "192.168.1.1", "192.168.1.2", "192.168.2.10", "192.168.77.1",
        "172.16.0.1", "172.18.5.1", "10.0.0.1", "10.10.10.9", "10.1.15.1",
    ];

    let locations = vec![
        "Москва, ЦОД-1", "СПб, Офис", "Казань, Узел A",
        "Екатеринбург, ЦОД", "Новосибирск, ЦОД", "Омск, Узел-7",
        "Москва, Офис Сормово",
    ];

    let metrics = vec![
        "cpu_usage",
        "memory_usage",
        "latency_ms",
        "packet_loss",
        "bandwidth_usage",
    ];

    let actions = vec![
        "CPU usage spike", "Memory usage high", "Bandwidth usage normal",
        "High latency detected", "Connection reset", "Packet loss detected",
    ];

    let mut rng = rand::thread_rng();

    TelemetryEvent {
        device_name: devices[rng.gen_range(0..devices.len())].to_string(),
        ip_address: ips[rng.gen_range(0..ips.len())].to_string(),
        location: locations[rng.gen_range(0..locations.len())].to_string(),
        metric_name: metrics[rng.gen_range(0..metrics.len())].to_string(),  // ⬅
        metric_value: rng.gen_range(0.0..100.0),
        action_description: actions[rng.gen_range(0..actions.len())].to_string(),
    }
}

async fn get_events(data: web::Data<SharedEvents>) -> impl Responder {
    println!("==> GET /events");

    let mut events = data.lock().unwrap();
    println!("==> В буфере {} событий", events.len());

    let response = events.clone();
    events.clear();

    println!("==> Буфер очищен");

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 FAKE TELEMETRY SERVER STARTED : http://127.0.0.1:10250/events");

    let events: SharedEvents = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            let event = generate_event();

            {
                let mut storage = events_clone.lock().unwrap();
                storage.push(event.clone());
            }

            println!("⚡ EVENT: {} [{}]", event.device_name, event.metric_name);
        }
    });

    let cleanup_events = events.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(300));
        loop {
            interval.tick().await;
            cleanup_events.lock().unwrap().clear();
            println!("🧹 Buffer cleared");
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(events.clone()))
            .route("/events", web::get().to(get_events))
    })
    .bind(("127.0.0.1", 10250))?
    .run()
    .await
}
