# Rust 教程 04：并行计算 rayon + ndarray（性能核心）

> 目标：用 `ndarray` 做 NumPy 式数组计算，用 `rayon` 把循环并行化——这是加速医疗影像计算的武器。

## ndarray：Rust 的 NumPy
```rust
use ndarray::Array2;
fn main() {
    let mut m = Array2::<f32>::from_shape_vec((2, 3),
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    println!("{:?}", m);
    println!("形状={:?} 元素数={}", m.shape(), m.len());
    m = &m * 2.0 + 1.0;                 // 逐元素向量化运算
    println!("变换后={:?}", m);
}
```

## rayon：把 `iter()` 换成 `par_iter()`
```rust
use rayon::prelude::*;
fn main() {
    let v: Vec<f32> = (0..1_000_000).map(|i| i as f32).collect();
    let sum: f64 = v.par_iter().map(|&x| x as f64).sum();   // 自动多线程
    println!("并行求和 = {}", sum);
}
```

## ndarray + rayon：并行图像算子
```toml
# Cargo.toml
ndarray = { version = "0.16", features = ["rayon"] }
rayon = "1"
```
```rust
use ndarray::Array2;
use rayon::prelude::*;

fn main() {
    let mut img = Array2::<f32>::from_shape_vec((512, 512),
        (0..512 * 512).map(|i| (i % 256) as f32).collect()).unwrap();

    if let Some(slice) = img.as_slice_mut() {        // 取连续底层内存
        slice.par_iter_mut().for_each(|x| *x = (*x - 128.0).abs());
    }
    println!("左上角 = {}", img[[0, 0]]);
}
```

## 应用：并行盒式模糊（邻域均值）
```rust
use ndarray::Array2;
use rayon::prelude::*;

fn box_blur(src: &Array2<f32>) -> Array2<f32> {
    let (h, w) = src.dim();
    let mut dst = Array2::<f32>::zeros((h, w));
    dst.indexed_iter_mut().par_bridge().for_each(|((y, x), out)| {
        let mut sum = 0.0_f32;
        let mut cnt = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;
                if ny >= 0 && ny < h as i32 && nx >= 0 && nx < w as i32 {
                    sum += src[[ny as usize, nx as usize]];
                    cnt += 1;
                }
            }
        }
        *out = sum / cnt as f32;
    });
    dst
}

fn main() {
    let img = Array2::<f32>::from_shape_vec((4, 4),
        (0..16).map(|i| i as f32).collect()).unwrap();
    println!("{:?}", box_blur(&img));
}
```

## 踩坑
- 并行只加速"彼此独立"的计算；有先后依赖的不能并行。
- `as_slice_mut()` 仅在内存连续时返回 `Some`；转置/切片后不连续会返回 `None`，先 `.to_owned()`。
- 并行闭包里用 `&` 借用大数据，别捕获会"移动"的大对象。

## 习题
1. 建 3x3 全 1 矩阵，并行把每个元素加 5。
2. 用 rayon 并行计算 `Vec<f32>` 所有元素的平方和。
