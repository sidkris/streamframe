use pyo3::prelude::*;
use std::collections::{HashMap, VecDeque};

//
// ============================================================
// GLOBAL STATS (Welford)
// ============================================================
//

struct GlobalStats {
    count: usize,
    mean: f64,
    m2: f64,
}

impl GlobalStats {
    fn new() -> Self {
        Self { count: 0, mean: 0.0, m2: 0.0 }
    }

    fn update(&mut self, x: f64) {
        self.count += 1;

        let delta = x - self.mean;
        self.mean += delta / self.count as f64;

        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }

    fn variance(&self) -> f64 {
        if self.count == 0 { 0.0 } else { self.m2 / self.count as f64 }
    }
}

//
// ============================================================
// ROLLING STATS (O(1))
// ============================================================
//

struct RollingStats {
    window: VecDeque<f64>,
    window_size: usize,
    sum: f64,
    sum_sq: f64,
}

impl RollingStats {
    fn new(window_size: usize) -> Self {
        Self {
            window: VecDeque::with_capacity(window_size),
            window_size,
            sum: 0.0,
            sum_sq: 0.0,
        }
    }

    fn update(&mut self, x: f64) {
        self.window.push_back(x);
        self.sum += x;
        self.sum_sq += x * x;

        if self.window.len() > self.window_size {
            if let Some(old) = self.window.pop_front() {
                self.sum -= old;
                self.sum_sq -= old * old;
            }
        }
    }

    fn mean(&self) -> f64 {
        if self.window.is_empty() {
            0.0
        } else {
            self.sum / self.window.len() as f64
        }
    }

    fn std(&self) -> f64 {
        let n = self.window.len();
        if n == 0 {
            return 0.0;
        }

        let mean = self.sum / n as f64;
        let var = (self.sum_sq / n as f64) - (mean * mean);

        var.max(0.0).sqrt()
    }
}

//
// ============================================================
// EWMA
// ============================================================
//

struct Ewma {
    value: f64,
    alpha: f64,
    initialized: bool,
}

impl Ewma {
    fn new(alpha: f64) -> Self {
        Self {
            value: 0.0,
            alpha,
            initialized: false,
        }
    }

    fn update(&mut self, x: f64) {
        if !self.initialized {
            self.value = x;
            self.initialized = true;
        } else {
            self.value = self.alpha * x + (1.0 - self.alpha) * self.value;
        }
    }

    fn get(&self) -> f64 {
        self.value
    }
}

//
// ============================================================
// COLUMN
// ============================================================
//

struct Column {
    values: Vec<f64>,
    global: GlobalStats,
    rolling: RollingStats,
    ewma: Ewma,
}

impl Column {
    fn new(window_size: usize, alpha: f64) -> Self {
        Self {
            values: Vec::new(),
            global: GlobalStats::new(),
            rolling: RollingStats::new(window_size),
            ewma: Ewma::new(alpha),
        }
    }

    fn append(&mut self, x: f64) {
        self.global.update(x);
        self.rolling.update(x);
        self.ewma.update(x);
        self.values.push(x);
    }

    fn last(&self) -> f64 {
        *self.values.last().unwrap_or(&0.0)
    }

    fn zscore(&self) -> f64 {
        let std = self.rolling.std();
        if std == 0.0 {
            return 0.0;
        }

        let mean = self.rolling.mean();
        let last = self.last();

        (last - mean) / std
    }
}

//
// ============================================================
// STREAM FRAME
// ============================================================
//

#[pyclass]
struct StreamFrame {
    columns: HashMap<String, Column>,
}

#[pymethods]
impl StreamFrame {
    #[new]
    fn new(col_names: Vec<String>, window_size: usize, alpha: f64) -> Self {
        let mut columns = HashMap::new();

        for name in col_names {
            columns.insert(name, Column::new(window_size, alpha));
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
        self.columns.get(&col).map(|c| c.global.mean).unwrap_or(0.0)
    }

    fn variance(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.global.variance()).unwrap_or(0.0)
    }

    fn last(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.last()).unwrap_or(0.0)
    }

    // ---- ROLLING ----

    fn rolling_mean(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.rolling.mean()).unwrap_or(0.0)
    }

    fn rolling_std(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.rolling.std()).unwrap_or(0.0)
    }

    fn zscore(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.zscore()).unwrap_or(0.0)
    }

    // ---- EWMA ----

    fn ewma(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.ewma.get()).unwrap_or(0.0)
    }
}

//
// ============================================================
// MODULE
// ============================================================
//

#[pymodule]
fn streamframe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StreamFrame>()?;
    Ok(())
}