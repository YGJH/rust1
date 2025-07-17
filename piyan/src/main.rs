// pi_chudnovsky_gpu.rs
// 用 Rust + rug (GMP/MPFR) 實作任意位數 π，並以 Rayon 做 CPU 並行
// GPU 加速示例註解部分，詳見註解中的說明

use rug::{Assign, Float, ops::PowAssign};
use rayon::prelude::*;

/// 計算 π 到 `digits` 位十進制精度
pub fn compute_pi_chudnovsky(digits: usize) -> Float {
    // 設定二進制精度：bits ≈ digits * log2(10) + 64 個額外位
    let bits = (digits as f64 * 3.32193).ceil() as u32 + 64;
    let precision = bits;

    // 計算需要的項數：每項約增加 14 位
    let terms = digits / 14 + 1;

    // 並行計算級數分子/分母並累加
    let sum = (0..terms).into_par_iter().map(|k| {
        // 公式中的 (-1)^k * (6k)! * (13591409 + 545140134 k)
        let mut num = Float::with_val(precision, 6 * k);
        num.factorial(); // num = (6k)!
        let coef = 13591409i128 + 545140134i128 * (k as i128);
        num *= coef;

        // 分母：(3k)! * (k!)^3 * 640320^(3k)
        let mut den = Float::with_val(precision, 3 * k);
        den.factorial(); // (3k)!
        let mut tmp = Float::with_val(precision, k);
        tmp.factorial(); // k!
        tmp.pow_assign(3); // (k!)^3
        den *= tmp;

        let mut p = Float::with_val(precision, 640320);
        p.pow_assign(3 * k); // 640320^(3k)
        den *= p;

        if k % 2 == 1 {
            num.neg_assign();
        }

        num / den
    }).reduce(|| Float::with_val(bits, 0), |a, b| {
        let mut r = a;
        r += b;
        r
    });

    // 常數 C = 12 / 640320^(3/2)
    let mut c = Float::with_val(precision, 640320);
    c.pow_assign(3); // ^3
    c.sqrt_assign(); // ^(3/2)
    c = Float::with_val(precision, 12) / c;

    // π = 1 / (sum * C)
    let mut inv = sum;
    inv *= c;
    let pi = Float::with_val(precision, 1) / inv;

    pi
}

fn main() {
    let digits = 10000; // 想要的位數
    let pi = compute_pi_chudnovsky(digits);
    // 輸出到標準輸出，%.Nf 控制小數位
    println!("π ≈ {:.prec$}", pi, prec = digits);

    // 如果要輸出到文件，請用：
    // let mut file = std::fs::File::create("pi.txt").unwrap();
    // write!(file, "{:.prec$}", pi, prec = digits).unwrap();
}

/*
GPU 加速說明：
- MPFR 運算必須在 CPU 上執行，無法直接 dispatch 到 GPU。
- 若要用 GPU 提速，可用 OpenCL/CUDA 做以下：
  1. 將每項級數的浮點近似轉為 double，並在 GPU 上計算 sum_k a_k (快速)
  2. 再回到 CPU，用 MPFR 對結果做高精度修正（Newton 校正）
- 或者自行實作 arbitrary-precision CUDA kernel，但需大量低階開發。
*/
