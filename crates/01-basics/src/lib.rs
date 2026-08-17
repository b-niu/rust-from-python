//! Stage 01: Rust 语言地基与核心概念示例

/// 01. 像素归一化示例函数（对应 basic-concepts.md）
pub fn normalize(val: f32) -> f32 {
    if val > 1.0 {
        1.0
    } else if val < 0.0 {
        0.0
    } else {
        val
    }
}

/// 02. 集合与迭代器计算：过滤并求均值（对应 collections-and-iterators.md）
pub fn mean_positive(values: &[f32]) -> Option<f32> {
    let positives: Vec<f32> = values.iter().copied().filter(|&x| x > 0.0).collect();
    if positives.is_empty() {
        None
    } else {
        let sum: f32 = positives.iter().sum();
        Some(sum / positives.len() as f32)
    }
}

/// 03. 医学图像结构体建模（对应 structs-and-enums.md）
#[derive(Debug, Clone, PartialEq)]
pub enum Modality {
    CT,
    MRI,
    Ultrasound,
}

#[derive(Debug, Clone)]
pub struct MedicalImage {
    pub width: usize,
    pub height: usize,
    pub modality: Modality,
    pub pixels: Vec<f32>,
}

impl MedicalImage {
    pub fn new(width: usize, height: usize, modality: Modality) -> Self {
        Self {
            width,
            height,
            modality,
            pixels: vec![0.0; width * height],
        }
    }

    pub fn pixel_count(&self) -> usize {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(1.5), 1.0);
        assert_eq!(normalize(-0.5), 0.0);
        assert_eq!(normalize(0.7), 0.7);
    }

    #[test]
    fn test_mean_positive() {
        let data = [-1.0, 2.0, 4.0, -3.0];
        assert_eq!(mean_positive(&data), Some(3.0));
        assert_eq!(mean_positive(&[-2.0]), None);
    }

    #[test]
    fn test_medical_image() {
        let img = MedicalImage::new(512, 512, Modality::CT);
        assert_eq!(img.pixel_count(), 262144);
        assert_eq!(img.modality, Modality::CT);
    }
}
