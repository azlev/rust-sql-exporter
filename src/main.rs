use std::fs;

use axum::{response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Query {
    query: Box<str>,
    interval: u64,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<Value> {
    let config: String = fs::read_to_string("sql.yaml").unwrap();

    let queries: Vec<Query> = serde_yaml::from_str(&config).unwrap();
    Json(serde_json::to_value(&queries).expect("Error!"))
}
