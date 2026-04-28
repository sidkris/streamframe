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
// COUNT-BASED ROLLING STATS
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
        if self.window.is_empty() { 0.0 }
        else { self.sum / self.window.len() as f64 }
    }

    fn std(&self) -> f64 {
        let n = self.window.len();
        if n == 0 { return 0.0; }

        let mean = self.sum / n as f64;
        let var = (self.sum_sq / n as f64) - (mean * mean);

        var.max(0.0).sqrt()
    }
}

//
// ============================================================
// TIME-BASED ROLLING STATS
// ============================================================
//

struct TimeRollingStats {
    window: VecDeque<(f64, i64)>,
    duration: i64,
    sum: f64,
    sum_sq: f64,
}

impl TimeRollingStats {
    fn new(duration: i64) -> Self {
        Self {
            window: VecDeque::new(),
            duration,
            sum: 0.0,
            sum_sq: 0.0,
        }
    }

    fn update(&mut self, x: f64, ts: i64) {
        self.window.push_back((x, ts));
        self.sum += x;
        self.sum_sq += x * x;

        while let Some(&(val, old_ts)) = self.window.front() {
            if ts - old_ts > self.duration {
                self.window.pop_front();
                self.sum -= val;
                self.sum_sq -= val * val;
            } else {
                break;
            }
        }
    }

    fn mean(&self) -> f64 {
        if self.window.is_empty() { 0.0 }
        else { self.sum / self.window.len() as f64 }
    }

    fn std(&self) -> f64 {
        let n = self.window.len();
        if n == 0 { return 0.0; }

        let mean = self.sum / n as f64;
        let var = (self.sum_sq / n as f64) - (mean * mean);

        var.max(0.0).sqrt()
    }

    fn rate(&self) -> f64 {
        if self.window.is_empty() || self.duration == 0 {
            return 0.0;
        }

        self.window.len() as f64 / self.duration as f64
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
        Self { value: 0.0, alpha, initialized: false }
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
    last: f64,
    global: GlobalStats,
    rolling: RollingStats,
    time_rolling: TimeRollingStats,
    ewma: Ewma,
}

impl Column {
    fn new(window_size: usize, alpha: f64, time_window: i64) -> Self {
        Self {
            last: 0.0,
            global: GlobalStats::new(),
            rolling: RollingStats::new(window_size),
            time_rolling: TimeRollingStats::new(time_window),
            ewma: Ewma::new(alpha),
        }
    }

    fn append(&mut self, x: f64, ts: i64) {
        self.last = x;
        self.global.update(x);
        self.rolling.update(x);
        self.time_rolling.update(x, ts);
        self.ewma.update(x);
    }

    fn zscore(&self) -> f64 {
        let std = self.rolling.std();
        if std == 0.0 {
            return 0.0;
        }

        (self.last - self.rolling.mean()) / std
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

// INTERNAL (Rust only)
impl StreamFrame {
    fn get_col(&self, col: &str) -> &Column {
        self.columns.get(col).expect("Column does not exist")
    }
}

#[pymethods]
impl StreamFrame {
    #[new]
    fn new(col_names: Vec<String>, window_size: usize, alpha: f64, time_window: i64) -> Self {
        let mut columns = HashMap::new();

        for name in col_names {
            columns.insert(name, Column::new(window_size, alpha, time_window));
        }

        StreamFrame { columns }
    }

    fn append(&mut self, row: HashMap<String, f64>, ts: i64) {
        for (key, value) in row {
            match self.columns.get_mut(&key) {
                Some(col) => col.append(value, ts),
                None => panic!("Column '{}' does not exist", key),
            }
        }
    }

    // GLOBAL
    fn mean(&self, col: String) -> f64 {
        self.get_col(&col).global.mean
    }

    fn variance(&self, col: String) -> f64 {
        self.get_col(&col).global.variance()
    }

    fn last(&self, col: String) -> f64 {
        self.get_col(&col).last
    }

    // COUNT
    fn rolling_mean(&self, col: String) -> f64 {
        self.get_col(&col).rolling.mean()
    }

    fn rolling_std(&self, col: String) -> f64 {
        self.get_col(&col).rolling.std()
    }

    fn zscore(&self, col: String) -> f64 {
        self.get_col(&col).zscore()
    }

    // TIME
    fn time_mean(&self, col: String) -> f64 {
        self.get_col(&col).time_rolling.mean()
    }

    fn time_std(&self, col: String) -> f64 {
        self.get_col(&col).time_rolling.std()
    }

    fn rate(&self, col: String) -> f64 {
        self.get_col(&col).time_rolling.rate()
    }

    // TREND
    fn ewma(&self, col: String) -> f64 {
        self.get_col(&col).ewma.get()
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