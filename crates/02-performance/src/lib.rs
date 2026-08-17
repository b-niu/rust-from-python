//! Stage 02: 性能与并行计算示例 (ndarray + rayon)

use ndarray::{Array2, Axis};
use rayon::prelude::*;

/// 使用 rayon 并行处理向量并进行非线性变换
pub fn parallel_relu(data: &mut [f32]) {
    data.par_iter_mut().for_each(|x| {
        if *x < 0.0 {
            *x = 0.0;
        }
    });
}

/// 使用 ndarray 计算二维图像的均值
pub fn image_mean(img: &Array2<f32>) -> Option<f32> {
    img.mean()
}

/// 并行计算二维矩阵各行之和
pub fn row_sums_parallel(matrix: &Array2<f32>) -> Vec<f32> {
    matrix
        .axis_iter(Axis(0))
        .into_par_iter()
        .map(|row| row.sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_parallel_relu() {
        let mut v = vec![-2.0, 1.5, -0.1, 4.0];
        parallel_relu(&mut v);
        assert_eq!(v, vec![0.0, 1.5, 0.0, 4.0]);
    }

    #[test]
    fn test_row_sums_parallel() {
        let m = arr2(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let sums = row_sums_parallel(&m);
        assert_eq!(sums, vec![6.0, 15.0]);
    }
}
