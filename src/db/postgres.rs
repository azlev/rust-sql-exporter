use tokio_postgres::Error;
use tokio_postgres::NoTls;

use crate::config::Query;
use crate::db::customerror::CustomError;
use crate::metric::{Metric, Row};

impl From<tokio_postgres::Error> for CustomError {
    fn from(err: Error) -> CustomError {
        CustomError::DBError(err.to_string())
    }
}

pub async fn query(conninfo: &str, query: &Query) -> Result<Metric, CustomError> {
    let (client, connection) = tokio_postgres::connect(conninfo, NoTls).await?;
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
    if rows.is_empty() {
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
