use std::{env, fmt, fs};

use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use postgres::{Client, Error, NoTls};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Query {
    query: String,
    interval: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Metric {
    name: String,
    labels: String,
    value: f64,
}

impl fmt::Display for Metric {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{0}{{{1}}} {2}", self.name, self.labels, self.value)
    }
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

    let mut ret = Vec::<String>::new();
    for q in queries.iter() {
        let item = query(q).unwrap();
        ret.push(item.to_string());
    }
    ret.join("\n")
}

fn query(query: &Query) -> Result<Metric, Error> {
    let conninfo = env::var("CONNINFO").unwrap();
    let _client = Client::connect(&conninfo, NoTls)?;
    let ret = Metric {
        name: query.query.clone(),
        labels: "".to_string(),
        value: 1.0,
    };
    Ok(ret)
}
