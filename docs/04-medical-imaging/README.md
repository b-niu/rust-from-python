# 医疗影像计算专题 (Medical Imaging with Rust)

本阶段聚焦于医学图像（CT、MRI、超声等）的高性能处理流水线，涵盖格式解析、3D 体素计算及与 PyTorch/MONAI 联动。

## 目录建议

- `dicom-parsing.md`：医学图像格式考：DICOM Tag 解析与像素解压缩
- `nifti-and-mhd.md`：NIfTI 与 MHD 格式读写与空间元数据处理
- `voxel-3d-simd.md`：三维体数据切片、插值与 SIMD 滤波加速
- `monai-pipeline.md`：端到端加速：构建接入 PyTorch/MONAI 的 Rust 预处理算子
