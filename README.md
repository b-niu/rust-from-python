# 🦀 Rust 从 Python 到影像与机器人 (Rust from Python)

> **“夫训诂者，正名审分，格物致知。”**  
> 专为 **Python 工程师**打造的 Rust 进阶手记与实战代码库，不设 C++ 前置包袱，融入语言学与词源训诂视角，直击**医疗影像分析**与**机器人实时系统**。

---

## 💡 本书特色与核心心法

1. **🐍 不问 C++，直指本心**：面向熟练 Python 开发者，以 `PyO3`、`maturin`、`NumPy ↔ ndarray` 零拷贝交互为核心，让 Rust 成为 Python 的高性能“涡轮增压器”。
2. **🔤 训诂正名，探求本义**：结合词源学（Etymology）与训诂考据，用“名实之辨”拆解所有权、借用检查与类型系统，让抽象的语法概念变得清晰通透。
3. **🏥 医疗影像**：3D 体素处理、DICOM/NIfTI 解析、SIMD 并行加速，与 PyTorch / MONAI 深度学习生态无缝集成。
4. **🤖 具身机器人**：nalgebra 运动学解算、LiDAR 点云与 IMU 传感器流水线、ROS 2 节点通信。

---

## 🗺️ 学习路线与知识地图

> **提示**：所有教程均采用**语义化路径组织**。你可以随时在任意阶段插入新文章，而无需改动现有文件路径。

### 阶段 1：语言地基与核心概念 (Basics)
- 📖 [自学大纲与心法](docs/outline.md)
- 📖 [1.1 基本概念与名实：所有权与借用](docs/01-language-basics/basic-concepts.md)
- 📖 [1.2 集合与迭代器：Rust 版的 NumPy 向量化](docs/01-language-basics/collections-and-iterators.md)
- 📖 [1.3 结构体与枚举：用类型系统建模医疗领域](docs/01-language-basics/structs-and-enums.md)
- 🦀 配套源码：[`crates/01-basics`](crates/01-basics/)

### 阶段 2：性能与系统编程 (Performance)
- 📖 [2.1 并行计算 rayon + ndarray：加速计算的利器](docs/02-performance-and-system/parallel-ndarray.md)
- 🦀 配套源码：[`crates/02-performance`](crates/02-performance/)

### 阶段 3：Rust ↔ Python 互操作 (PyO3 & Interop)
- 📖 [3.0 互操作专题总览与规划](docs/03-python-interop/README.md)
- 🦀 配套源码：[`crates/03-py-fastmath`](crates/03-py-fastmath/)

### 阶段 4：医疗影像算法加速 (Medical Imaging)
- 📖 [4.0 医疗影像专题规划与算子设计](docs/04-medical-imaging/README.md)

### 阶段 5：医疗机器人与实时控制 (Robotics)
- 📖 [5.0 机器人专题规划与 ROS 2 架构](docs/05-robotics/README.md)

### 阶段 6：工程化、基准与合规 (Engineering)
- 📖 [6.1 工程化落地：测试 / 基准 / 发布 Wheel](docs/06-engineering-and-production/engineering-testing-benchmarks.md)
- 🦀 配套源码：[`crates/06-engineering`](crates/06-engineering/)

---

## 🚀 快速上手与运行

### 1. 运行所有 Rust 测试
```bash
cargo test --workspace
```

### 2. 运行性能基准测试 (Criterion)
```bash
cargo bench --package stage06-engineering
```

### 3. 将 Rust 扩展编译为 Python 模块
```bash
# 安装 maturin
pip install maturin numpy

# 一键编译并安装进当前 Python 环境
maturin develop --release
```

在 Python 中调用：
```python
import numpy as np
import py_fastmath

arr = np.array([[-1.0, 2.5], [-3.0, 4.0]], dtype=np.float32)
py_fastmath.relu_inplace(arr)
print(arr)  # [[0.0, 2.5], [0.0, 4.0]]
```

---

## 📜 开源协议
本项目采用 [MIT License](LICENSE) 开源。
