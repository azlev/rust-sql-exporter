use std::{env, fs};

use axum::{
    extract::State,
    http::header,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::time::{self, Duration, Instant};
use tokio_postgres::NoTls;

use rust_sql_exporter::customerror::CustomError;
use rust_sql_exporter::metric::{Metric, MetricType, Row, SharedMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Query {
    query: String,
    metric: String,
    #[serde(rename = "type")]
    type_: MetricType,
    help: String,
    interval: Option<u64>,
}

#[derive(Clone)]
struct AppState {
    conninfo: String,
    queries_sync: Vec<Query>,
    metrics_async: SharedMap,
}

#[tokio::main]
async fn main() {
    // extract envvars just here
    let conninfo = env::var("RSE_CONNINFO").unwrap();
    let conffile = env::var("RSE_CONFIG").unwrap();
    let bind_address = env::var("RSE_ADDRESS").unwrap_or("0.0.0.0:3000".to_string());

    let config: String = fs::read_to_string(conffile).unwrap();
    let queries: Vec<Query> = serde_yaml::from_str(&config).unwrap();

    let mut queries_async: Vec<Query> = Vec::new();
    let mut queries_sync: Vec<Query> = Vec::new();
    for q in queries.iter() {
        match q.interval {
            None => queries_sync.push(q.clone()),
            Some(_i) => queries_async.push(q.clone()),
        };
    }

    let query_results: SharedMap = SharedMap::new();

    let appstate = AppState {
        conninfo: conninfo.to_string(),
        queries_sync: queries_sync,
        metrics_async: query_results.clone(),
    };

    tokio::spawn(async move {
        let mut last_tick: Vec<Instant> = Vec::new();

        // populate last_tick with all interval "expired"
        for q in queries_async.iter() {
            last_tick.push(Instant::now() - Duration::from_secs(q.interval.unwrap()));
        }

        loop {
            for (i, q) in queries_async.iter().enumerate() {
                let d = last_tick[i] + Duration::from_secs(q.interval.unwrap());
                if Instant::now() > d {
                    let query_result = query(&conninfo, &q).await;
                    match query_result {
                        Ok(metric) => query_results.insert(metric),
                        Err(e) => eprintln!("Error in metric '{}': {}", q.metric, e.to_string()),
                    }
                    last_tick[i] = Instant::now();
                }
                let mut interval = time::interval(Duration::from_secs(1));
                interval.tick().await; // Wait for the next tick
            }
        }
    });

    let app = Router::new()
        .route("/metrics", get(handler_metrics))
        .route("/", get(handler_main))
        .with_state(appstate);

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

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

async fn handler_metrics(State(appstate): State<AppState>) -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8; escaping=values",
        )],
        body(appstate).await,
    )
}

async fn body(appstate: AppState) -> String {
    // build the synchronous response (non interval-based metrics)
    let mut ret = Vec::<String>::new();
    for q in appstate.queries_sync.iter() {
        let item = query(&appstate.conninfo, q).await;
        match item {
            Ok(metric) => ret.push(metric.to_string()),
            Err(e) => eprintln!("Error in metric '{}': {}", q.metric, e.to_string()),
        };
    }
    // add interval-based metrics
    appstate.metrics_async.extract_result(&mut ret);

    ret.push("".to_string()); // add an empty string to insert a newline at the end with the join below
    ret.join("\n")
}

async fn query(conninfo: &String, query: &Query) -> Result<Metric, CustomError> {
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
