# Rust 教程 01：Rust 是什么，以及你必须先建立的几个基本概念

> 适用对象：Python 熟练，但**没有计算机科班背景、不懂 C/C++**。
> 配套目标：用 Rust 重写 Python 里的性能热点，服务医疗影像分析与医疗机器人。
> 用法：按「目标 → 核心概念 → 代码 → 踩坑 → 习题」读，代码都能直接复制运行。

---

## 目标（学完这一篇你能做什么）

1. 说清楚 Rust 为什么快、为什么安全，以及它和你目标的关联。
2. 能在本地跑起一段 Rust 代码（不追求写大程序）。
3. 理解 5 个最基础、也最反直觉的概念：`let`/`mut`、类型、函数、**所有权/借用**、**Option/Result**。
4. 建立正确的心智模型，为后续「把 Rust 函数暴露给 Python 调用」打地基。

---

## 0. 先建立大局观：Rust 对你意味着什么

你写 Python 时，解释器在背后帮你做了很多事：变量类型随时变、内存自动回收、出错就抛异常。代价是**慢**和**不可预测**——对「实时机器人控制」「大批量影像预处理」这种场景，这就成了瓶颈。

Rust 的思路完全不同：它在**编译阶段**（代码运行之前）就把内存安全和并发安全检查完。好处是：
- **速度接近 C**——没有垃圾回收（GC）的停顿，机器人控制循环不会被"卡一下"。
- **内存安全**——编译期就杜绝"用了已释放的内存"这类 bug，对医疗软件是刚需。
- **和 Python 是搭档不是替代**——你继续用 Python 写业务逻辑，只把算得慢的部分用 Rust 重写，再从 Python 调用（后续教程讲 PyO3）。

一句话：**Rust 给你的 Python 装涡轮，而你几乎不用懂底层硬件。**

> 关键差异：Python 是"边读边跑"（解释执行），Rust 是"先编译成机器码再跑"。所以 Rust 跑之前会先报一堆红字错误——别怕，那是编译器在替你兜底，相当于有个严格但耐心的老师提前帮你改 bug。

---

## 核心概念 1：`let` 与 `mut` —— 变量默认不能改

Python 里变量随便改：`x = 5; x = 6` 毫无问题。Rust 默认**变量一旦赋值就不能改**，要改必须显式写 `mut`（mutable 的缩写）。

```rust
fn main() {
    let x = 5;          // 不可变（默认，最安全）
    // x = 6;          // ❌ 编译报错：cannot assign twice to immutable variable
    let mut y = 5;      // 可变：你明确声明"我要改它"
    y = 6;              // ✅
    println!("x = {}, y = {}", x, y);
}
```

**为什么你的目标需要它**：医疗影像里，一张 CT 数据是上百万个像素。如果代码默认不允许意外修改，就能避免"某处不小心改了原始图像，下游全算错"的诡异 bug。Rust 逼你把"会改"和"不会改"说清楚。

---

## 核心概念 2：类型 —— 把 Python 的 `dtype` 写到了表面上

你用 NumPy 时已经接触过类型：`np.float32`、`np.int16`。Rust 把这件事**提前到编译期，并且强制**：

```rust
fn main() {
    let a: f32 = 1.0;     // 单精度浮点，等价于 np.float32（省内存，GPU 友好）
    let b = 2.0_f32;      // 另一种写法，明确指定单精度
    let c = 2.0;          // 不写时，Rust 默认推断为 f64（双精度，更准但占内存）
    let n: i16 = 1024;    // 16 位整数，等价于 np.int16（CT 灰度常这么存）

    println!("a={}, b={}, c={}, n={}", a, b, c, n);
}
```

大部分时候 Rust 能**自动推断**类型（像上面 `c`），不需要你全写上。但你要知道类型存在，并且**两种不同精度不能直接混算**（比如 `f32 + f64` 会报错）——这其实是在保护你，避免医疗计算里因精度混杂产生静默误差。

---

## 核心概念 3：函数 —— 和 Python 几乎一样，多了一个"返回类型"

Python：`def add(a, b): return a + b`
Rust：参数和返回值都要标类型。

```rust
fn add(a: f32, b: f32) -> f32 {   // -> f32 表示返回值类型
    a + b                          // 最后一行就是返回值（不用写 return）
}

fn main() {
    let r = add(1.5, 2.5);
    println!("1.5 + 2.5 = {}", r);
}
```

> 小技巧：Rust 函数里**最后一行的表达式自动作为返回值**，不加 `return`、也不加 `;`。加了 `;` 反而会变成"不返回值"。初学常踩，记住即可。

---

## 核心概念 4（最重要）：所有权与借用 —— Rust 的内存安全核心

这是 Rust 和 Python 最大的不同，也是你唯一需要"换脑子"的地方。用**借书**来类比：

- **所有权（ownership）**：一本书（一块内存）同一时间只能有一个主人。
- **移动（move）**：你把书送给别人，主人就变成对方，你手里没了。
- **借用（borrow）**：你把书借给别人看，对方看完还给你，主人还是你。

### 4.1 移动：把数据"交给"别人

```rust
fn main() {
    let img = vec![1.0_f32, 2.0, 3.0]; // 一批像素数据，img 是主人
    let img2 = img;                     // 所有权"移动"给 img2（书送人了）
    // println!("{:?}", img);          // ❌ img 已不再是主人，不能用
    println!("{:?}", img2);            // ✅ img2 才是主人
}
```

Python 里 `img2 = img` 是"两份引用指向同一本书"；Rust 里是"书过户了"。这样编译器能保证：同一份内存**永远只有一个主人负责释放**，不会重复释放或提前释放。

### 4.2 借用：只是"借去看"，还回来还能用

实际中你常想把数据传给函数处理，但还想要回原数据。用 `&`（取地址/借用的符号）即可：

```rust
// &[f32] 表示"借一串 f32 来只读"，不拿走所有权
fn mean(data: &[f32]) -> f32 {
    let sum: f32 = data.iter().sum();
    sum / data.len() as f32
}

fn main() {
    let pixels = vec![1.0_f32, 2.0, 3.0, 4.0];
    let m = mean(&pixels);     // 借给函数；&pixels 就是"把书借出去"
    println!("均值 = {}", m);
    println!("原数据还在：{:?}", pixels);  // ✅ 借完还回来了，照样能用
}
```

**为什么你的目标需要它**：机器人控制里，传感器数据要同时喂给"记录日志""控制算法""可视化"多个模块。借用让多个模块**安全地共享同一份数据而互不破坏**，且零拷贝、不占额外内存。

---

## 核心概念 5：Option 与 Result —— 没有"空指针崩溃"，没有"偷偷抛异常"

Python 里常见两件事：(a) 一个值可能是 `None`（比如没读到像素）；(b) 出错时 `raise` 异常。这两种在大型医疗系统里都很危险——一个没处理的 `None` 或异常可能让诊断出错。

Rust 的思路：**把"可能没有"和"可能出错"变成类型本身**，逼你一定要处理。

### 5.1 Option<T>：明确"可能有，也可能没有"

```rust
// 返回 Option<f32>：要么 Some(值)，要么 None（明确说"没有"）
fn first_pixel(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        None
    } else {
        Some(data[0])
    }
}

fn main() {
    let empty: Vec<f32> = vec![];
    match first_pixel(&empty) {
        Some(v) => println!("第一像素 = {}", v),
        None    => println!("图像是空的，跳过处理"),  // 必须写这一支，否则编译不过
    }
}
```

注意：你**必须**写 `None` 那一支，编译器才让你过。这就从语言层面消灭了"忘了处理空值导致崩溃"。

### 5.2 Result<T, E>：错误是"一个值"，不是"一声惊雷"

Python 用 `try/except`；Rust 用 `Result`——函数要么返回 `Ok(正常结果)`，要么返回 `Err(错误说明)`，由调用方决定怎么处理。

```rust
// Result<成功类型, 错误类型>。这里错误用字符串描述（后面会教更好的做法）
fn normalize(data: &[f32]) -> Result<Vec<f32>, String> {
    if data.is_empty() {
        return Err("图像为空，无法归一化".to_string());
    }
    let max = data.iter().cloned().fold(0.0_f32, |acc, v| acc.max(v));
    let out: Vec<f32> = data.iter().map(|&v| v / max).collect();
    Ok(out)
}

fn main() {
    let pixels = vec![0.0_f32, 50.0, 100.0];
    match normalize(&pixels) {
        Ok(norm) => println!("归一化结果 = {:?}", norm),
        Err(e)   => println!("出错了：{}", e),
    }
}
```

**为什么你的目标需要它**：医疗软件里"出错必须被显式处理"是硬性要求。Rust 让任何可能失败的运算（读 DICOM、除零、设备断开）都逃不过你的处理，不会悄悄带病运行。

---

## 最小可运行例子（综合）：一个安全的"影像均值 + 归一化"

把上面概念串起来，写一个小而完整的程序。它就是你将来要"加速"的典型计算——也是将来用 PyO3 暴露给 Python 的那类函数雏形。

```rust
// 借用切片，计算均值（不可变借用，安全共享）
fn mean(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        return None;
    }
    let sum: f32 = data.iter().sum();
    Some(sum / data.len() as f32)
}

// 归一化到 0~1，显式返回可能出错的 Result
fn normalize(data: &[f32]) -> Result<Vec<f32>, String> {
    if data.is_empty() {
        return Err("图像为空".to_string());
    }
    let max = data.iter().cloned().fold(0.0_f32, |acc, v| acc.max(v));
    Ok(data.iter().map(|&v| v / max).collect())
}

fn main() {
    let scan = vec![0.0_f32, 128.0, 255.0, 64.0];

    match mean(&scan) {
        Some(m) => println!("影像均值 = {}", m),
        None    => println!("没有数据"),
    }

    match normalize(&scan) {
        Ok(out) => println!("归一化后 = {:?}", out),
        Err(e) => println!("错误：{}", e),
    }
}
```

运行方式（本地已装 Rust 时）：
```bash
cargo new demo01 && cd demo01
# 把上面代码粘进 src/main.rs，覆盖原内容
cargo run
```
> 不想装环境也能试：打开 https://play.rust-lang.org ，粘贴代码点 "Run"。

---

## 踩坑记录（初学必看）

1. **"cannot assign twice to immutable variable"**：你改了没标 `mut` 的变量。解决：在 `let` 后加 `mut`。
2. **"borrow of moved value"**：数据被"移动"走后你还想用。解决：用 `&` 借用而不是转移所有权（参考 4.2）。
3. **"mismatched types: expected f32, found f64"**：精度混用。统一成同一种（如都写 `_f32` 或都标 `: f32`）。
4. **最后一行加了 `;` 导致"不返回值"**：返回值的表达式别加 `;`。
5. **`match` 漏写分支编译不过**：`Option`/`Result` 的 `match` 必须把所有情况写上，这是 Rust 在保护你。

---

## 回顾习题（不看书能写出来才算懂）

1. 用 `let mut` 写一个计数器，循环加 1 共 10 次，最后打印结果。
2. 写一个函数 `max_of(data: &[f32]) -> Option<f32>`，返回数组最大值；空数组返回 `None`。
3. 解释给"不懂编程的朋友"：Rust 的"移动"和"借用"分别像生活中的什么？
4. 为什么 Rust 强制你处理 `None` 和 `Err`，对你做医疗软件是好事？（用自己的话写 2 句）

---

## 下一篇预告

教程 02：**集合与迭代器**——`Vec`（动态数组）、`HashMap`，以及 `iter().map().filter().collect()` 这套"Rust 版 NumPy 向量化"写法，开始接触和医疗数组计算直接相关的工具。

---

*本篇配合《Rust 自学大纲》阶段 1 使用。建议把"踩坑"和"习题答案"补进你自己的笔记。*
