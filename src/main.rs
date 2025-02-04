use std::fs;

use axum::{
    http::header,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Query {
    query: Box<str>,
    interval: u64,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/metrics", get(handler_metrics))
        .route("/", get(handler_main));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler_main() -> Html<&'static str> {
    Html(
        "<html>
<head><title>SQL Exporter</title></head>
  <body>
    <h1>SQL Exporter</h1>
    <p><a href='/metrics'>Metrics</a></p>
  </body>
</html>",
    )
}

async fn handler_metrics() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8; escaping=values",
        )],
        body(),
    )
}

fn body() -> String {
    let config: String = fs::read_to_string("sql.yaml").unwrap();

    let queries: Vec<Query> = serde_yaml::from_str(&config).unwrap();
    let json = serde_json::to_value(&queries).expect("Error in to_value");
    serde_json::to_string_pretty(&json).expect("Error in to_string_pretty")
}
