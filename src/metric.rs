use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Write;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetricType {
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
            MetricType::Histogram => "histogram",
            MetricType::Summary => "summary",
        };
        write!(fmt, "{}", r)
    }
}

#[derive(Debug)]
pub struct Row {
    pub labels: Vec<(String, String)>,
    pub value: f64,
}

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub rows: Vec<Row>,
    pub type_: MetricType,
    pub help: String,
}

impl fmt::Display for Metric {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "# HELP {0} {1}\n", self.name, self.help)?;
        write!(fmt, "# TYPE {0} {1}\n", self.name, self.type_)?;
        let mut tmp = String::new();
        for row in self.rows.iter() {
            write!(tmp, "{}", self.name)?;
            write!(tmp, "{{")?;
            if row.labels.len() > 0 {
                for t in row.labels.iter() {
                    write!(tmp, "{0}=\"{1}\", ", t.0, t.1)?;
                }
                tmp.pop();
                tmp.pop();
            }
            write!(tmp, "}}")?;
            write!(tmp, " {0}\n", row.value)?;
        }
        tmp.pop();
        write!(fmt, "{}", tmp)?;
        Ok(())
    }
}
