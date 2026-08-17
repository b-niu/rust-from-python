# Rust 自学大纲：用 Rust 加速 Python，服务医疗影像分析与医疗机器人

> 目标读者：已经熟练使用 Python（数据/影像/机器人方向），希望把 Python 中的性能热点用 Rust 重写以获得接近 C/C++ 的速度，同时保证内存安全。
> 用途：这是一份**给自己写教程的骨架**——每个阶段都按「目标 / 核心概念 / 最小可运行例子 / 踩坑记录 / 回顾习题」组织，方便你边学边填。

---

## 0. 先想清楚：你为什么学 Rust（定方向）

你的三个核心诉求，决定了一条和「学通用 Rust」不一样的路线：

| 诉求 | 对学习内容的影响 |
|------|------------------|
| **加速 Python 代码** | 重点不是写完整 Rust 应用，而是写**被 Python 调用的扩展模块**（PyO3 + maturin）。数据零拷贝传递（NumPy ↔ ndarray）是命门。 |
| **医疗影像分析** | 重点是数值计算、数组/图像处理、DICOM 解析、GPU 加速，以及和 PyTorch/NumPy 生态对接。 |
| **医疗机器人** | 重点是实时性、ROS 2 集成、控制循环、运动学与传感器 I/O（串口/网络/点云）。 |

**核心心法**：Rust 不追求「替代 Python」，而是做 Python 的「涡轮增压器」。80% 的业务逻辑（数据加载、可视化、实验管理、训练）继续用 Python，只有**计算密集、且已用 `py-spy` 确认是瓶颈**的部分才用 Rust 重写。

**给自己写教程的模板**（每个子主题都套用）：
```
## 主题：xxx
- 目标：学完能做什么
- 核心概念：（用自己的话写 3-5 条）
- 最小可运行例子：（5-30 行）
- 踩坑：（编译报错、所有权、GIL 等，边学边记）
- 回顾习题：（不看书能写出来才算懂）
```

---

## 1. 前置条件与工具链（第 0 周，半天搞定）

**安装清单**：
- `rustup` + 稳定版 Rust（`cargo`、`rustc`）
- 编辑器：VS Code + `rust-analyzer` 插件（必须，Rust 的语法提示/重构极度依赖它）
- Python 侧：`pip install maturin numpy`（建议用 venv 或 conda 隔离）
- 机器人方向额外：`libclang-dev`（Ubuntu）、后续按需安装 ROS 2（Humble/Jazzy）
- 影像方向额外：`cmake` + C++ 编译器（`dicom-pixeldata` 的 `gdcm` 特性需要）

**验证安装**：
```bash
cargo --version
rustc --version
python -c "import numpy; print(numpy.__version__)"
pip show maturin | head -2
```

**必装 cargo 工具（后续会用）**：
```bash
cargo install cargo-watch cargo-expand cargo-flamegraph
# cargo-watch: 改代码自动重编译；cargo-expand: 展开宏看真相；flamegraph: 性能火焰图
```

> 踩坑预警：在中国大陆访问 crates.io 慢，配置镜像：
> 在 `~/.cargo/config.toml` 写入 `[source.crates-io] replace-with = "rsproxy-sparse"` 等镜像源（搜索「cargo 国内镜像」取最新配置）。

---

## 2. 学习路线总览（6 个阶段，约 14–16 周）

| 阶段 | 主题 | 周数 | 里程碑（checkpoint） |
|------|------|------|----------------------|
| 1 | Rust 语言基础 | 2–3 | 用 Rust 重写一个纯函数并跑通测试 |
| 2 | 性能与系统编程 | 2 | 用 rayon 并行 ndarray，速度对比 Python |
| 3 | **Rust ↔ Python 互操作（核心）** | 2–3 | 第一个 PyO3 扩展替换 Python 热点 |
| 4 | 医疗影像分析专题 | 3 | 加速一条影像预处理 pipeline |
| 5 | 医疗机器人专题 | 3 | Rust 写 ROS 2 节点跑通控制循环 |
| 6 | 工程化、基准与合规 | 1–2 | 发布 wheel + 基准报告 + 测试 |

> 节奏建议：每天 1–1.5 小时，周末做 checkpoint 项目。前紧后松——阶段 3 之后你会越学越快，因为模式重复了。

---

## 阶段 1：Rust 语言基础（第 1–3 周）

**目标**：能读懂并写出正确的 Rust，过掉「所有权/借用」这一关（这是 90% 初学者卡住的地方）。

核心概念（按优先级）：
1. **所有权（ownership）+ 借用（borrow）+ 生命周期（lifetime）**——Rust 没有 GC，靠这套规则在编译期保证内存安全。这是你最重要的地基，值得花最多时间。
2. 基础类型、`match`、结构体/`enum`、方法（`impl`）
3. `Result<T, E>` 与 `Option<T>`——Rust 不用异常，用返回值处理错误
4. 集合（`Vec`/`HashMap`）、迭代器（`iter().map().filter().collect()`）
5. 泛型与 trait（先理解「trait 像接口」即可，进阶的 `trait bound`/`where` 后续再深抠）

推荐资源：
- **《The Rust Programming Language》（官方免费书，必读前 15 章）**：https://doc.rust-lang.org/book/
- 《Rustlings》交互式小练习（边读边练，强烈推荐）：`cargo install rustlings`
- 中文：《Rust 程序设计语言》中文版（同上书的翻译）

最小可运行例子（先感受所有权）：
```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // 借用，不转移所有权
    println!("'{}' 的长度是 {}", s, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Checkpoint 项目：挑一个你熟悉的纯计算函数（如计算 Hausdorff 距离、或列表去重统计），用 Rust 重写并写单元测试（`cargo test`）。

---

## 阶段 2：性能与系统编程（第 4–5 周）

**目标**：理解 Rust 为什么快，并掌握并行与数值计算的基础设施。

核心概念：
1. **零成本抽象（zero-cost abstraction）**：泛型、`Iterator` 编译后和手写循环一样快。
2. **`unsafe` 与 FFI 基础**：理解为什么有时需要 `unsafe`（直接操作指针/调 C 库），以及它不破坏整体安全。
3. **并行：`rayon`**——把 `iter()` 换成 `par_iter()` 就能并行，几乎零成本。
4. **数值计算：`ndarray`**——Rust 版的 NumPy，N 维数组。
5. **性能剖析**：`cargo flamegraph`、`py-spy`（分析 Python 瓶颈）、`std::hint::black_box`。

最小可运行例子（rayon 并行求和）：
```rust
use rayon::prelude::*;
fn main() {
    let v: Vec<f64> = (0..1_000_000).map(|i| i as f64).collect();
    let sum: f64 = v.par_iter().sum(); // 自动多线程
    println!("sum = {}", sum);
}
```

Checkpoint 项目：用 `ndarray` + `rayon` 实现一个大矩阵逐元素运算（如高斯滤波核展开），对比 Python/NumPy 耗时，记录加速比。

---

## 阶段 3：Rust ↔ Python 互操作（核心，第 6–8 周）

> 这是你整个学习路线的「主菜」。前面所有基础，都是为了让你能写出被 Python 无缝调用的高性能扩展。

核心概念：
1. **PyO3**：用 `#[pyfunction]`/`#[pyclass]` 把 Rust 函数/结构体暴露成 Python 可调用对象。
2. **maturin**：构建并打包成可直接 `import` 的扩展模块（`.so`/`.pyd`），并生成标准 wheel。
3. **零拷贝 NumPy 互操作（最关键）**：用 `rust-numpy` 的 `PyReadonlyArray` / `PyReadwriteArray`，让 Rust 直接借 NumPy 数组内存，**不复制**。`&PyReadonlyArray2<f64>` → `ArrayView2<f64>`。
4. **释放 GIL**：纯计算函数用 `Python::acquire_gil(false)` 或 `Python::with_gil` + `allow_threads`，让多个 Rust 线程真正并行（绕过 Python 全局锁）。
5. **错误映射**：Rust 的 `Result` 自动变成 Python 异常；自定义异常用 `#[pyclass(extends=PyException)]`。
6. **类型标注**：用 `pyo3-stub-gen` 或手写 `.pyi` stub，让 Python 侧有补全。

最小可运行例子（零拷贝加速 NumPy）：
```toml
# Cargo.toml
[lib]
name = "fast_math"
crate-type = ["cdylib"]
[dependencies]
pyo3 = { version = "0.28", features = ["extension-module"] }
numpy = "0.23"
ndarray = "0.16"
```
```rust
use numpy::{PyReadwriteArray2, PyArray2};
use ndarray::s;
use pyo3::prelude::*;

#[pyfunction]
fn add_one(mut arr: PyReadwriteArray2<f64>) -> PyResult<()> {
    // 零拷贝：直接在原 NumPy 数组上就地修改
    let mut view = arr.as_array_mut();
    view += 1.0;
    Ok(())
}
#[pymodule]
fn fast_math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_one, m)?)?;
    Ok(())
}
```
```bash
# 构建并装进当前 venv
maturin develop --release
```
```python
import numpy as np, fast_math
a = np.ones((1000, 1000))
fast_math.add_one(a)   # a 已被就地加 1
```

Checkpoint 项目：**把你 Python 代码里一个真实热点**（比如影像归一化、特征提取、距离计算）用 Rust+PyO3 重写并替换，写一个 `bench.py` 用 `timeit` 量化加速比（目标 ≥5×）。记得用 `py-spy` 先确认它确实是瓶颈。

> 进阶技巧（写进笔记）：`#[pyfunction(acquire_gil = false)]` 让函数不持有 GIL；大数组务必用 `PyReadonlyArray`/`PyReadwriteArray` 而非 `Vec` 传参（避免复制）。

---

## 阶段 4：医疗影像分析专题（第 9–11 周）

**目标**：把 Rust 接入真实医疗影像栈——DICOM 解析、图像处理算子、与 PyTorch/NumPy 共享内存。

核心概念与库：
1. **`dicom-rs`**：纯 Rust 的 DICOM 解析。关键 crate：`dicom`（聚合）+ `dicom-pixeldata`（解码像素，可转 `ndarray`/`image`）。
   ```rust
   use dicom::object::open_file;
   let obj = open_file("img.dcm")?;
   let patient = obj.element_by_name("PatientName")?.to_str()?;
   // 像素转 ndarray（features=["ndarray"]）
   let px = obj.decode_pixel_data()?.to_ndarray::<f32>()?;
   ```
2. **图像处理算子**：卷积、重采样、插值、形态学——在 `ndarray` 上实现，用 `rayon` 并行，再用 PyO3 暴露给 Python。
3. **与 PyTorch 对接**：两种方式——(a) 经 NumPy 中转（最稳）；(b) 用 `torch` 的共享内存 / `tch` crate 直接调用 libtorch（进阶）。
4. **GPU 加速**：`wgpu`（跨平台 Vulkan/Metal/DX12）写计算着色器；CUDA 在 Rust 里生态不成熟（`rust-cuda` 偏实验），优先 `wgpu` 或回退到调用 Python 侧的 PyTorch/CuPy。
5. **基准**：`criterion` 做严谨微基准（比 `timeit` 更可靠，给出置信区间）。

Checkpoint 项目：选一条你真实的**影像预处理 pipeline**（如 CT 窗宽窗位调整 + 重采样 + 归一化），把最慢的 1–2 步用 Rust 重写，做成 `pip install` 的 wheel，写 `criterion` 基准报告对比原 Python 实现。

---

## 阶段 5：医疗机器人专题（第 12–14 周）

**目标**：用 Rust 写 ROS 2 节点，处理实时控制循环与传感器数据，弥补 Python 在实时性上的短板。

核心概念与库：
1. **`r2r`**（ROS 2 异步 Rust 绑定，0.9.5，支持 Humble/Jazzy）：不依赖 colcon，`cargo build` 即可；需先 `source /opt/ros/<distro>/setup.sh` 且装 `libclang-dev`。
   ```rust
   use r2r::QosProfile;
   let ctx = r2r::Context::create()?;
   let mut node = r2r::Node::create(ctx, "rust_node", "")?;
   let sub = node.subscribe::<r2r::sensor_msgs::msg::Image>("/camera", QosProfile::default())?;
   ```
2. **运动学与线性代数：`nalgebra`**（Rust 的 Eigen 等价物）+ `nalgebra-lapack`（分解/求逆）。
3. **控制循环**：用 `r2r` 的 timer + `spin_once` 跑固定频率控制律；纯计算部分放 Rust，实时性远好于 rclpy。
4. **传感器 I/O**：串口（`serialport` crate）、网络（`tokio`）、点云处理（`nalgebra` + 自定义结构）。
5. **与 Python 训练/推理栈结合**：Rust 负责实时感知与控制，Python 负责离线训练/可视化/高层决策，经 ROS 2 topic 或 PyO3 扩展通信。

Checkpoint 项目：写一个 Rust ROS 2 节点，订阅相机/关节状态 topic，在固定频率下跑一个简单控制律（如 PID 或运动学逆解），把结果发布出去；用 `ros2 topic echo` 验证。

---

## 阶段 6：工程化、基准与合规（第 15–16 周）

**目标**：让你的 Rust 扩展达到「可交付」水平。

清单：
1. **测试**：`cargo test` + `pytest` 跨语言测试；用 `cargo miri test` 抓未定义行为（UB）。
2. **基准**：`criterion` 出带误差棒的对比图；记录与原 Python 的加速比。
3. **发布 wheel**：`maturin build --release`，用 `cibuildwheel` 在 CI 里产出多平台（x86/ARM、多 Python 版本）wheel。
4. **CI**：GitHub Actions 跑测试 + 构建 + 基准；用 `abi3` 特性兼容多 CPython 版本。
5. **内存安全即合规优势**：医疗软件常涉及 IEC 62304 等流程。Rust 的编译期内存/数据竞争保证，是写「安全关键代码」的强背书——但**务必明确：Rust 安全 ≠ 医疗合规认证**，认证仍需走正规流程与文档，本大纲不构成法律/法规建议。
6. **可复现**：固定 `Cargo.lock`，记录 Rust/Python 版本，写清 README 与 `.pyi` 类型桩。

最终 Capstone（建议）：把你阶段 4 + 阶段 5 的产出合并——一个 Rust 扩展同时服务「影像预处理加速」和「机器人控制」，统一成你的个人工具箱 crate，配 `criterion` 基准报告与使用示例。

---

## 3. 推荐资源汇总

| 类型 | 名称 | 说明 |
|------|------|------|
| 官方书 | 《The Rust Programming Language》 | 前 15 章必读，免费 |
| 练习 | Rustlings | 交互式小练习，`cargo install rustlings` |
| 进阶书 | 《Rust for Rustaceans》(Jon Gjengset) |  ownership/trait/FFI 深入 |
| 互操作 | PyO3 官方指南 | https://pyo3.rs/main |
| 构建 | maturin | `pip install maturin` |
| 数值 | ndarray / nalgebra | Rust 的 NumPy / Eigen |
| 并行 | rayon | 一行改成并行 |
| 影像 | dicom-rs | 纯 Rust DICOM |
| 机器人 | r2r | ROS 2 异步绑定 |
| 基准 | criterion | 严谨微基准 |
| 剖析 | cargo-flamegraph / py-spy | 找瓶颈 |

---

## 4. 学习原则（贴墙）

1. **先测后优化**：永远先用 `py-spy` 找到真实瓶颈，再动手写 Rust。否则可能是白忙。
2. **小步替换**：别想着把整个 Python 项目翻译成 Rust。一次换一个函数，验证了再换下一个。
3. **零拷贝优先**：跨语言传大数组用 `PyReadonlyArray`，复制是性能的隐形杀手。
4. **编译期报错是朋友**：Rust 编译器的报错信息极好，读它、照做，别烦躁。
5. **每个主题都写进教程笔记**：按第 0 节的模板，学完就能沉淀成你自己的手册。

---

*本大纲可随学习进度迭代。建议每完成一个 checkpoint，就在对应阶段下方补充「我实际踩的坑」与「可复用的代码片段」。*