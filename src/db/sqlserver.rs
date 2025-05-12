use std::io::Error as IOError;
use tiberius::{Client, Config};
use tiberius::error::Error as SQLError;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use crate::config::Query;
use crate::db::customerror::CustomError;
use crate::metric::{Metric, Row};

impl From<IOError> for CustomError {
    fn from(err: IOError) -> CustomError {
        CustomError::DBError(err.to_string())
    }
}

impl From<SQLError> for CustomError {
    fn from(err: SQLError) -> CustomError {
        CustomError::DBError(err.to_string())
    }
}

pub async fn query(conninfo: &String, query: &Query) -> Result<Metric, CustomError> {
    let config = Config::from_ado_string(&conninfo)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;
    let stream = client.query(&query.query, &[]).await?;
    let ret = Metric {
        name: query.metric.clone(),
        rows: Vec::new(),
        type_: query.type_.clone(),
        help: query.help.clone(),
    };
    let rows = stream.into_row().await?.unwrap();
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
