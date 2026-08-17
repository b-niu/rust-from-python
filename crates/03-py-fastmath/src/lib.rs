//! Stage 03: PyO3 扩展模块与 NumPy 零拷贝加速

use numpy::{PyArray2, PyReadwriteArray2};
use pyo3::prelude::*;

/// 零拷贝：就地将 NumPy 二维数组中的负值截断为 0 (ReLU)
#[pyfunction]
fn relu_inplace(mut arr: PyReadwriteArray2<f32>) -> PyResult<()> {
    let mut view = arr.as_array_mut();
    view.mapv_inplace(|x| if x < 0.0 { 0.0 } else { x });
    Ok(())
}

/// 纯 Rust 加速计算两点间欧氏距离
#[pyfunction]
fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Python 模块定义
#[pymodule]
fn py_fastmath(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(relu_inplace, m)?)?;
    m.add_function(wrap_pyfunction!(euclidean_distance, m)?)?;
    Ok(())
}
