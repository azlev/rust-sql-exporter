use crate::config::Query;
use crate::db::customerror::CustomError;
use crate::metric::Metric;

use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub async fn query(conninfo: &String, query: &Query) -> Result<Metric, CustomError> {
    let config = Config::from_ado_string(&conninfo)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let ret = Metric {
        name: query.metric.clone(),
        rows: Vec::new(),
        type_: query.type_.clone(),
        help: query.help.clone(),
    };
    Ok(ret)
}
