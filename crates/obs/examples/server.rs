

use opentelemetry::global;
use nebulafx_obs::{SystemObserver, init_obs};
use std::time::{Duration, SystemTime};
use tracing::{Level, error, info, instrument};

#[tokio::main]
async fn main() {
    let obs_conf = Some("http://localhost:4317".to_string());
    let _guard = init_obs(obs_conf).await;
    let span = tracing::span!(Level::INFO, "main");
    let _enter = span.enter();
    info!("Program starts");
    // Simulate the operation
    tokio::time::sleep(Duration::from_millis(100)).await;
    run("service-demo".to_string()).await;
    info!("Program ends");
}

#[instrument(fields(bucket, object, user))]
async fn run(service_name: String) {
    let start_time = SystemTime::now();
    info!("Log module initialization is completed service_name: {:?}", service_name);

    // Record Metrics
    let meter = global::meter("nebulafx");
    let request_duration = meter.f64_histogram("s3_request_duration_seconds").build();
    request_duration.record(
        start_time.elapsed().unwrap().as_secs_f64(),
        &[opentelemetry::KeyValue::new("operation", "run")],
    );

    match SystemObserver::init_process_observer().await {
        Ok(_) => info!("Process observer initialized successfully"),
        Err(e) => error!("Failed to initialize process observer: {:?}", e),
    }

    put_object("bucket".to_string(), "object".to_string(), "user".to_string()).await;
    info!("Logging is completed");
    tokio::time::sleep(Duration::from_secs(2)).await;
    info!("Program run ends");
}

#[instrument(fields(bucket, object, user))]
async fn put_object(bucket: String, object: String, user: String) {
    let start_time = SystemTime::now();
    info!("Starting put_object operation time: {:?}", start_time);

    let meter = global::meter("nebulafx");
    let request_duration = meter.f64_histogram("s3_request_duration_seconds").build();
    request_duration.record(
        start_time.elapsed().unwrap().as_secs_f64(),
        &[opentelemetry::KeyValue::new("operation", "put_object")],
    );

    info!(
        "Starting PUT operation content: bucket = {}, object = {}, user = {},start_time = {}",
        bucket,
        object,
        user,
        start_time.elapsed().unwrap().as_secs_f64()
    );

    // Simulate the operation
    tokio::time::sleep(Duration::from_millis(100)).await;

    info!("PUT operation completed");
}
