# Rust ↔ Python 互操作专题 (PyO3 & Maturin)

本阶段聚焦于如何将 Rust 编写的高性能算法封装为 Python 扩展模块，实现无缝调用与零拷贝数据传递。

## 目录建议

- `pyo3-quickstart.md`：PyO3 与 Maturin 极速上手：构建第一个 Python C-Extension
- `numpy-zero-copy.md`：NumPy 与 ndarray 零拷贝内存映射（`PyReadonlyArray` / `PyReadwriteArray`）
- `gil-management.md`：释放 GIL 突破性能枷锁：多线程并行计算
- `error-and-types.md`：异常与类型系统桥接：Result 映射与 Python 类型提示 (.pyi)
