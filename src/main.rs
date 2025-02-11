use std::{env, fs};

use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rust_sql_exporter::customerror::CustomError;
use rust_sql_exporter::metric::{Metric, MetricType, Row};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    query: String,
    metric: String,
    #[serde(rename = "type")]
    type_: MetricType,
    help: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/metrics", get(handler_metrics))
        .route("/", get(handler_main));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

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
        body().await,
    )
}

async fn body() -> String {
    let conninfo = env::var("RSE_CONFIG").unwrap();
    let config: String = fs::read_to_string(conninfo).unwrap();

    let queries: Vec<Query> = serde_yaml::from_str(&config).unwrap();

    let mut ret = Vec::<String>::new();
    for q in queries.iter() {
        let item = query(q).await;
        match item {
            Ok(metric) => ret.push(metric.to_string()),
            Err(e) => eprintln!("Error in metric '{}': {}", q.metric, e.to_string()),
        };
    }
    ret.join("\n")
}

async fn query(query: &Query) -> Result<Metric, CustomError> {
    let conninfo = env::var("RSE_CONNINFO").unwrap();
    let (client, connection) = tokio_postgres::connect(&conninfo, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut ret = Metric {
        name: query.metric.clone(),
        rows: Vec::new(),
        type_: query.type_.clone(),
        help: query.help.clone(),
    };
    let rows = client.query(&query.query, &[]).await?;
    if rows.len() == 0 {
        return Err(CustomError::EmptyVec);
    }
    let l: usize = rows[0].len();
    for row in rows.iter() {
        let mut r = Row {
            labels: Vec::new(),
            // by design, the value is always the last column
            value: row.try_get(l - 1)?,
        };
        for i in 0..(l - 1) {
            let s: String = row.get(i);
            let t = (row.columns()[i].name().to_string(), s);
            r.labels.push(t);
        }
        ret.rows.push(r);
    }
    Ok(ret)
}
