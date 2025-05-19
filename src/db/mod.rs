use crate::config::Query;
use crate::metric::Metric;

pub mod customerror;

use customerror::CustomError;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "mssql")]
pub mod mssql;

pub async fn query(conninfo: &String, query: &Query) -> Result<Metric, CustomError> {
    if conninfo.starts_with("server=") {
        proxy_sqlserver_query(conninfo, query).await
    } else {
        postgres::query(conninfo, query).await
    }
}

#[cfg(feature = "mssql")]
async fn proxy_sqlserver_query(conninfo: &String, query: &Query) -> Result<Metric, CustomError> {
    mssql::query(conninfo, query).await
}

#[cfg(not(feature = "mssql"))]
async fn proxy_sqlserver_query(_conninfo: &String, _query: &Query) -> Result<Metric, CustomError> {
    panic!("Enable SQL Server feature at compile time");
}
