# Rust 教程 02：集合与迭代器（Rust 版"NumPy 向量化"）

> 目标：掌握 `Vec`/`HashMap` 与迭代器链 `iter().map().filter().collect()`，这是后面做数组计算的基础。

## Vec：动态数组
```rust
fn main() {
    let mut v: Vec<f32> = vec![1.0, 2.0, 3.0];
    v.push(4.0);                 // 追加
    let first = v[0];            // 下标访问
    println!("{:?} 第一个={} 长度={}", v, first, v.len());
}
```

## 迭代器链（零拷贝、可并行）
```rust
fn main() {
    let v: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let out: Vec<f32> = v.iter()        // 迭代 &f32
        .filter(|&&x| x > 2.0)          // 过滤
        .map(|&x| x * 10.0)             // 变换
        .collect();                     // 收集成 Vec
    println!("{:?}", out);              // [30, 40, 50]
}
```

## enumerate / zip
```rust
fn main() {
    let pixels = vec![10.0_f32, 20.0, 30.0];
    for (i, &p) in pixels.iter().enumerate() {
        println!("第{}个像素 = {}", i, p);
    }
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];
    let summed: Vec<i32> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    println!("{:?}", summed);           // [11, 22, 33]
}
```

## HashMap（类似 Python dict）
```rust
use std::collections::HashMap;
fn main() {
    let mut m: HashMap<&str, f32> = HashMap::new();
    m.insert("CT", 1.0);
    m.insert("MRI", 2.0);
    match m.get("CT") {                 // 取值返回 Option，必须处理"没有"
        Some(v) => println!("CT 权重 = {}", v),
        None => println!("没有这个模态"),
    }
}
```

## 应用：一行统计影像均值/最大值
```rust
fn main() {
    let scan: Vec<f32> = vec![0.0, 128.0, 255.0, 64.0];
    let mean = scan.iter().sum::<f32>() / scan.len() as f32;
    let max = scan.iter().cloned().fold(0.0_f32, |a, x| a.max(x));
    println!("均值={} 最大={}", mean, max);
}
```

## 踩坑
- 要改元素用 `iter_mut()`（给 `&mut T`），只读用 `iter()`。
- `collect()` 常需标注类型（如 `: Vec<f32>`），否则编译器不知道收集成什么。
- `v[99]` 越界会 **直接 panic**；想安全用 `v.get(99)` 拿 `Option`。

## 习题
1. 用迭代器把 `vec![1,2,3,4,5]` 中偶数平方，收集成 `Vec<i32>`。
2. 用 `iter().sum()` 算一张 `Vec<f32>` 影像的总和与均值。
