use std::collections::HashMap;

use axum::http::StatusCode;
use axum::{
    extract::Query,
    routing::{get, MethodRouter},
    Router,
};

use prometheus::{core::Desc, proto::MetricFamily, Encoder, Registry, TextEncoder};
use serde::Deserialize;

use crate::collector;

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

fn metrics_string(f: Vec<MetricFamily>) -> String {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];

    match encoder.encode(&f, &mut buffer) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to encode metrics: {}", e);
            return String::from("Failed to encode metrics");
        }
    }

    String::from_utf8(buffer.clone()).unwrap_or(String::from("Failed to encode metrics"))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ProbeParams {
    account_id: Option<String>,
    target: Option<String>,
}

pub fn probe() -> Router {
    async fn handler(Query(params): Query<ProbeParams>) -> (StatusCode, String) {
        let registry = Registry::new();

        let desc = Desc::new(
            String::from("near_da"),
            String::from("Metrics for NEAR Protocol"),
            vec![],
            HashMap::new(),
        )
        .unwrap();

        let target = match params.target {
            Some(target) => target,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    String::from("Missing required parameter: target"),
                )
            }
        };

        let account_id = match params.account_id {
            Some(account_id) => account_id,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    String::from("Missing required parameter: account_id"),
                )
            }
        };

        let mut n = collector::ViewAccountMetricsCollector::new(desc, target, account_id);

        n.fetch().await;

        match registry.register(Box::new(n)) {
            Ok(_) => println!("Registered collector"),
            Err(e) => {
                println!("Failed to register collector: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to register collector"),
                );
            }
        };

        (StatusCode::OK, metrics_string(registry.gather()))
    }
    route("/probe", get(handler))
}

pub fn metrics() -> Router {
    async fn handler() -> String {
        let metric_families = prometheus::gather();
        metrics_string(metric_families)
    }
    route("/metrics", get(handler))
}

pub fn healthz() -> Router {
    async fn handler() -> String {
        String::from("OK")
    }
    route("/healthz", get(handler))
}
