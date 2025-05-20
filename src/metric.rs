use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::sync::{Arc, Mutex};

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

#[derive(Clone, Debug)]
pub struct Row {
    pub labels: Vec<(String, String)>,
    pub value: f64,
}

#[derive(Clone, Debug)]
pub struct Metric {
    pub name: String,
    pub rows: Vec<Row>,
    pub type_: MetricType,
    pub help: String,
}

impl fmt::Display for Metric {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "# HELP {0} {1}", self.name, self.help)?;
        writeln!(fmt, "# TYPE {0} {1}", self.name, self.type_)?;
        let mut tmp = String::new();
        for row in self.rows.iter() {
            write!(tmp, "{}", self.name)?;
            write!(tmp, "{{")?;
            if !row.labels.is_empty() {
                for t in row.labels.iter() {
                    write!(tmp, "{0}=\"{1}\", ", t.0, t.1)?;
                }
                tmp.pop();
                tmp.pop();
            }
            write!(tmp, "}}")?;
            writeln!(tmp, " {0}", row.value)?;
        }
        tmp.pop();
        write!(fmt, "{}", tmp)?;
        Ok(())
    }
}

// https://draft.ryhl.io/blog/shared-mutable-state/
#[derive(Clone)]
pub struct SharedMap {
    inner: Arc<Mutex<SharedMapInner>>,
}

struct SharedMapInner {
    data: HashMap<String, Metric>,
}

impl SharedMap {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SharedMapInner {
                data: HashMap::new(),
            })),
        }
    }

    pub fn insert(&self, value: Metric) {
        let mut lock = self.inner.lock().unwrap();
        lock.data.insert(value.name.to_string(), value);
    }

    pub fn extract_result(&self, vec: &mut Vec<String>) {
        let lock = self.inner.lock().unwrap();
        for (_, v) in lock.data.iter() {
            vec.push(v.to_string());
        }
    }
}

impl Default for SharedMap {
    fn default() -> Self {
        Self::new()
    }
}
