# Rust 教程 03：结构体、枚举与方法（用类型建模你的领域）

> 目标：用 `struct` 建模数据（如医学图像、机器人状态），用 `enum` 表达"有限种可能"（如模态、传感器状态）。

## struct + impl（类似 Python dataclass + 方法）
```rust
struct MedicalImage {
    width: usize,
    height: usize,
    pixels: Vec<f32>,
    modality: String,
}

impl MedicalImage {
    fn new(width: usize, height: usize, modality: &str) -> Self {
        Self { width, height, pixels: vec![0.0; width * height], modality: modality.to_string() }
    }
    fn mean(&self) -> f32 {                 // &self = 只读方法
        self.pixels.iter().sum::<f32>() / self.pixels.len() as f32
    }
}

fn main() {
    let img = MedicalImage::new(256, 256, "CT");
    println!("{} 图像均值 = {}", img.modality, img.mean());
}
```

## enum：有限种可能（比字符串安全，编译器帮你查错）
```rust
enum Modality { CT, MRI, XRay, Ultrasound }

struct Point3 { x: f64, y: f64, z: f64 }

fn main() {
    let m = Modality::CT;
    match m {
        Modality::CT => println!("这是 CT"),
        Modality::MRI => println!("这是 MRI"),
        _ => println!("其他模态"),
    }
    let p = Point3 { x: 1.0, y: 2.0, z: 3.0 };
    println!("点 = ({}, {}, {})", p.x, p.y, p.z);
}
```

## enum 携带数据（表达"结果/消息"）
```rust
enum SensorReading { Ok(f64), Disconnected }

fn report(r: &SensorReading) {
    match r {
        SensorReading::Ok(v) => println!("读数 = {}", v),
        SensorReading::Disconnected => println!("⚠ 传感器断开"),
    }
}

fn main() {
    report(&SensorReading::Ok(3.14));
    report(&SensorReading::Disconnected);
}
```

## 应用：机器人状态 + 平移
```rust
struct RobotState { position: Point3, joint_angles: Vec<f64> }

impl RobotState {
    fn translate(&mut self, dx: f64, dy: f64, dz: f64) {  // &mut self = 可改
        self.position.x += dx;
        self.position.y += dy;
        self.position.z += dz;
    }
}

fn main() {
    let mut s = RobotState { position: Point3 { x: 0.0, y: 0.0, z: 0.0 },
                             joint_angles: vec![0.0, 0.0] };
    s.translate(1.0, 0.5, 0.0);
    println!("新位置 z = {}", s.position.z);
}
```

## 踩坑
- 只读方法用 `&self`，要改字段必须 `&mut self`，否则编译报错。
- 想从别的模块访问字段，加 `pub`（如 `pub width: usize`）。
- `match` 必须覆盖所有 enum 变体，用 `_` 兜底其余。

## 习题
1. 给 `MedicalImage` 加 `normalize(&mut self)`，把像素缩放到 0~1。
2. 定义 `enum Command { Move(f64), Stop, Home }`，用 `match` 打印每种指令含义。
