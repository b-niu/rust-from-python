//! Stage 06: 工程化测试与基准规范 (对应 engineering-testing-benchmarks.md)

/// 将像素值归一化/截断到 [0.0, 1.0] 区间
pub fn normalize_clamp(x: f32) -> f32 {
    if x > 1.0 {
        1.0
    } else if x < 0.0 {
        0.0
    } else {
        x
    }
}

/// 批量归一化切片
pub fn normalize_slice(data: &mut [f32]) {
    for val in data.iter_mut() {
        *val = normalize_clamp(*val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_clamps() {
        assert_eq!(normalize_clamp(1.5), 1.0);
        assert_eq!(normalize_clamp(-0.3), 0.0);
        assert_eq!(normalize_clamp(0.4), 0.4);
    }

    #[test]
    fn test_normalize_preserves_zero() {
        assert_eq!(normalize_clamp(0.0), 0.0);
    }
}
