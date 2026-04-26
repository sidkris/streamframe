use pyo3::prelude::*;
use std::collections::HashMap;

//
// ---- COLUMN ----
//

struct Column {
    values: Vec<f64>,
    count: usize,
    mean: f64,
    m2: f64,
}

impl Column {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    fn append(&mut self, x: f64) {
        self.count += 1;

        // Welford Algo
        let delta = x - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;

        self.values.push(x);
    }

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
    fn new(col_names: Vec<String>) -> Self {
        let mut columns = HashMap::new();

        for name in col_names {
            columns.insert(name, Column::new());
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

    fn mean(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.mean()).unwrap_or(0.0)
    }

    fn variance(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.variance()).unwrap_or(0.0)
    }

    fn last(&self, col: String) -> f64 {
        self.columns.get(&col).map(|c| c.last()).unwrap_or(0.0)
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