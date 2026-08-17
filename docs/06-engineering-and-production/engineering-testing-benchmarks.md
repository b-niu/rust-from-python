# Rust 自学教程 08 — 工程化：测试 / 基准 / 发布 wheel

> 目标：把你写的 Rust 加速函数变成"可信 + 可量化 + 可分发给 Python 同事"的产物。这是大纲第 6 阶段（工程化与合规）的落地。

---

## 1. 这堆工具名叫啥

> 🔤 **test** /test/ 测试：来自古法语 *tester*「检验、试金（assay）」——本意是炼金时验证成色。

> 🔤 **assert** /əˈsɜːt/ 断言：拉丁语 *asserere*「断言、主张、把……据为己有」。代码里 = "我主张这个结果必然成立"。

> 🔤 **criterion** /kraɪˈtɪəriən/ 基准（基准测试库）：希腊语 *kritērion*「判断的标准」，词根 *kritēs* = 裁判。所以用 criterion 跑出来的就是"裁判用的尺子"。

> 🔤 **bench** /bɛntʃ/：*benchmark*「基准」= 木工 *bench*（工作台）+ *mark*（刻痕）。原指石匠在固定台面上刻的参考标记——可复现、可对比。

> 🔤 **clippy**：Rust 的 lint（代码体检）工具。名字取「帮你把代码理干净的助手」之意——像回形针📎一样"夹"住问题，也有人说联想 Office 里那个曲别针助手 Clippy。

> 🔤 **wheel** /wiːl/ 轮子：Python 的二进制包格式，接替旧格式 *egg*。取名自"轮子好滚"——比 egg 更好分发、安装更快。

---

## 2. 单元测试：`cargo test`

```rust
// src/lib.rs
/// 把像素值夹到 [0,1]，正是教程 01 的雏形
pub fn normalize(x: f32) -> f32 {
    if x > 1.0 { 1.0 } else if x < 0.0 { 0.0 } else { x }
}

#[cfg(test)]                 // 只在 `cargo test` 时编译
mod tests {
    use super::*;

    #[test]
    fn test_normalize_clamps() {
        assert_eq!(normalize(1.5), 1.0);   // 上限
        assert_eq!(normalize(-0.3), 0.0);  // 下限
        assert_eq!(normalize(0.4), 0.4);   // 原值不变
    }

    #[test]
    fn test_normalize_preserves_zero() {
        assert_eq!(normalize(0.0), 0.0);
    }
}
```

```bash
cargo test
# running 2 tests ... test result: ok. 2 passed; 0 failed
```

失败示例：`assert_eq!(normalize(2.0), 0.0)` 会红，并直接告诉你期望值与实际值。

---

## 3. 基准测试：`criterion`（量化"Rust 比 Python 快多少"）

```toml
# Cargo.toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "normalize_bench"
harness = false            # 关掉内置 bench，交给 criterion
```

```rust
// benches/normalize_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use rust_accel::normalize;   // 你的库

fn bench_normalize(c: &mut Criterion) {
    c.bench_function("normalize_1000", |b| {
        b.iter(|| {
            let v: Vec<f32> = (0..1000).map(|i| (i % 7) as f32 / 3.0).collect();
            v.iter().map(|&x| normalize(x)).count()
        })
    });
}
criterion_group!(benches, bench_normalize);
criterion_main!(benches);
```

```bash
cargo bench               # 输出每组的平均耗时/方差，可对比 Python 同款
```

> 用途：把 criterion 数字和 Python `timeit` 数字放一起，就是你向导师/同事证明"上 Rust 值不值"的证据。

---

## 4. 代码体检：`cargo clippy`

```bash
cargo clippy --all-targets -- -W clippy::all
```

常见它会揪出：
```rust
let _ = v.len();          // ⚠ needless_range_loop / 可改用 iter()
if x == true { ... }      // ⚠ 直接写 if x { }
```
修掉这些，代码更地道、也更不易藏 bug。

---

## 5. 打包给 Python：用 `maturin` 出 wheel

```toml
# Cargo.toml（在教程 05 基础上）
[lib]
name = "medical_accel"
crate-type = ["cdylib"]          # 编译成可被 Python 加载的扩展

[dependencies]
pyo3 = { version = "0.28", features = ["extension-module"] }
ndarray = "0.16"
```

```bash
maturin develop            # 编译并直接装进当前 Python 环境（开发用）
maturin build --release    # 在 target/wheels/ 下生成 .whl（发布用）
maturin publish            # 上传到 PyPI（需先配置 token）
```

生成的 `target/wheels/medical_accel-0.1.0-cp311-cp311-linux_x86_64.whl` 就是那个"轮子"——同事 `pip install` 即可，无需装 Rust。

---

## 6. 一条龙最小 CI（GitHub Actions 片段）

```yaml
# .github/workflows/ci.yml
- run: cargo test
- run: cargo clippy -- -D warnings     # 有 clippy 警告就红
- run: cargo bench -- --quick          # 守速度回归
```

---

## 7. 踩坑

- `cargo test` 只跑 `src/`；集成测试放 `tests/`，基准放 `benches/` 且 `harness=false`。
- criterion 第一次跑会编译较久，属正常。
- `maturin develop` 必须在**正确的 Python 虚拟环境**里执行，否则装错地方。
- 发布 wheel 前，确认 `pyproject.toml` 里 `name`/`version` 已设，且 `pyo3` 开了 `extension-module`（否则和 pip 的 Python 冲突）。
- 医疗软件若要过审（FDA/CE），`cargo test` 覆盖 + `clippy` 零警告 + 基准报告，都是你质量证据链的一环。

---

## 8. 回顾习题

1. 给教程 04 的 `box_blur` 写一个 `#[test]`，验证"全 1 图像模糊后还是全 1"。
2. 用 `criterion` 给 `box_blur` 和一段等价的 Python（双重 for 循环）计时，算加速比。
3. 故意写 `let y = x.clone(); if y == x {}` 跑 `clippy`，看它给什么建议并改正。
4. 把 `maturin build --release` 产出的 `.whl` 名字抄下来，说清每个字段（名字/版本/Python 标签/平台）代表啥。
