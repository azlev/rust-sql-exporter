use std::fmt::Write;
use std::{env, fmt, fs};

use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, NoTls};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

impl fmt::Display for MetricType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let r = match self {
            MetricType::Counter => "counter",
            MetricType::Gauge => "gauge",
            MetricType:: Histogram => "histogram",
            MetricType::Summary => "summary",
        };
        write!(fmt, "{}", r)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    query: String,
    interval: u64,
    metric: String,
    #[serde(rename="type")]
    type_: MetricType,
}

#[derive(Debug)]
struct Metric {
    name: String,
    labels: Vec<(String, String)>,
    value: f64,
    type_: MetricType,
}

impl fmt::Display for Metric {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "# HELP {0}\n", self.name)?;
        write!(fmt, "# TYPE {0} {1}\n", self.name, self.type_)?;
        write!(fmt, "{}", self.name)?;
        write!(fmt, "{{")?;
        if self.labels.len() > 0 {
            let mut tmp = String::new();
            for t in self.labels.iter() {
                write!(tmp, "{0}=\"{1}\", ", t.0, t.1)?;
            }
            tmp.pop();
            tmp.pop();
            write!(fmt, "{}", tmp)?;
        }
        write!(fmt, "}}")?;
        write!(fmt, " {0}", self.value)
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
        body().await,
    )
}

async fn body() -> String {
    let config: String = fs::read_to_string("sql.yaml").unwrap();

    let queries: Vec<Query> = serde_yaml::from_str(&config).unwrap();

    let mut ret = Vec::<String>::new();
    for q in queries.iter() {
        let item = query(q).await.unwrap();
        ret.push(item.to_string());
    }
    ret.join("\n")
}

async fn query(query: &Query) -> Result<Metric, Error> {
    let conninfo = env::var("CONNINFO").unwrap();
    let (client, connection) = tokio_postgres::connect(&conninfo, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let rows = client.query(&query.query, &[]).await?;
    let l: usize = rows[0].len();
    let mut ret = Metric {
        name: query.metric.clone(),
        labels: Vec::new(),
        // by design, the value is always the last column
        value: rows[0].get(l - 1),
        type_: query.type_.clone(),
    };
    for i in 0..(l - 1) {
        let s: String = rows[0].get(i);
        let t = (rows[0].columns()[i].name().to_string(), s);
        ret.labels.push(t);
    }
    Ok(ret)
}
