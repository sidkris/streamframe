use pyo3::prelude::*;
use std::collections::{HashMap, VecDeque};

//
// ---- COLUMN ----
//

struct Column {
    // full history
    values: Vec<f64>,

    // global stats (Welford)
    count: usize,
    mean: f64,
    m2: f64,

    // rolling window
    window: VecDeque<f64>,
    window_size: usize,
    rolling_sum: f64,
    rolling_sum_sq: f64,
}

impl Column {
    fn new(window_size: usize) -> Self {
        Self {
            values: Vec::new(),
            count: 0,
            mean: 0.0,
            m2: 0.0,
            window: VecDeque::with_capacity(window_size),
            window_size,
            rolling_sum: 0.0,
            rolling_sum_sq: 0.0,
        }
    }

    fn append(&mut self, x: f64) {
        // ---- Welford (global stats) ----
        self.count += 1;

        let delta = x - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;

        // ---- Rolling window ----
        self.window.push_back(x);
        self.rolling_sum += x;
        self.rolling_sum_sq += x * x;

        if self.window.len() > self.window_size {
            if let Some(old) = self.window.pop_front() {
                self.rolling_sum -= old;
                self.rolling_sum_sq -= old * old;
            }
        }

        // ---- Store (optional) ----
        self.values.push(x);
    }

    // ---- GLOBAL ----

    fn mean(&self) -> f64 {
        self.mean
    }

    fn variance(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        self.m2 / self.count as f64
    }

    fn last(&self) -> f64 {
        *self.values.last().unwrap_or(&0.0)
    }

    // ---- ROLLING ----

    fn rolling_mean(&self) -> f64 {
        let n = self.window.len();
        if n == 0 {
            return 0.0;
        }
        self.rolling_sum / n as f64
    }

    fn rolling_std(&self) -> f64 {
        let n = self.window.len();
        if n == 0 {
            return 0.0;
        }

        let mean = self.rolling_sum / n as f64;
        let variance = (self.rolling_sum_sq / n as f64) - (mean * mean);

        // numerical safety
        variance.max(0.0).sqrt()
    }
}

//
// ---- STREAM FRAME ----
//

#[pyclass]
struct StreamFrame {
    columns: HashMap<String, Column>,
}

#[pymethods]
impl StreamFrame {
    #[new]
    fn new(col_names: Vec<String>, window_size: usize) -> Self {
        let mut columns = HashMap::new();

        for name in col_names {
            columns.insert(name, Column::new(window_size));
        }

        StreamFrame { columns }
    }

    fn append(&mut self, row: HashMap<String, f64>) {
        for (key, value) in row {
            if let Some(col) = self.columns.get_mut(&key) {
                col.append(value);
            }
        }
    }

    // ---- GLOBAL ----

    fn mean(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.mean()).unwrap_or(0.0)
    }

    fn variance(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.variance()).unwrap_or(0.0)
    }

    fn last(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.last()).unwrap_or(0.0)
    }

    // ---- ROLLING ----

    fn rolling_mean(&self, col: String) -> f64 {
        self.columns
            .get(&col)
            .map(|c| c.rolling_mean())
            .unwrap_or(0.0)
    }

    fn rolling_std(&self, col: String) -> f64 {
        self.columns
            .get(&col)
            .map(|c| c.rolling_std())
            .unwrap_or(0.0)
    }
}

//
// ---- MODULE ----
//

#[pymodule]
fn streamframe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StreamFrame>()?;
    Ok(())
}