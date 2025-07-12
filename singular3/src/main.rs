use std::io::{self, Read, Write, BufWriter};
use std::collections::VecDeque;



fn main() {
    // 快速讀取所有輸入
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();

    // 解析參數
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let a: usize = it.next().unwrap().parse().unwrap();
    let b: usize = it.next().unwrap().parse().unwrap();
    let mut g: u64 = it.next().unwrap().parse().unwrap();
    let x: u64   = it.next().unwrap().parse().unwrap();
    let y: u64   = it.next().unwrap().parse().unwrap();
    let z: u64   = it.next().unwrap().parse().unwrap();

    // 列滑窗的隊列陣列：每一列保留 (row_index, min_value)
    let cols = m - b + 1;
    let mut col_dqs: Vec<VecDeque<(usize,u64)>> = vec![VecDeque::new(); cols];

    // 用來存當前行的高度，以及這行的每個 b 長度滑窗最小值
    let mut row_h = vec![0u64; m];
    let mut row_min = vec![0u64; cols];

    let mut ans: u64 = 0;

    // 逐行生成 h，算行內長 b 滑窗最小並直接餵給 col_dqs
    for i in 0..n {
        // 先把整行的 h 值生成出來
        for j in 0..m {
            if i == 0 && j == 0 {
                // g 已經是 g0
            } else {
                g = (g.wrapping_mul(x).wrapping_add(y)) % z;
            }
            row_h[j] = g;
        }
        // 單調佇列算行滑窗 min
        let mut dq = VecDeque::new();
        for j in 0..m {
            // push new
            while let Some(&(_, val)) = dq.back() {
                if val >= row_h[j] {
                    dq.pop_back();
                } else {
                    break;
                }
            }
            dq.push_back((j, row_h[j]));
            // pop 過期
            if let Some(&(idx, _)) = dq.front() {
                if idx + b <= j {
                    dq.pop_front();
                }
            }
            // 計算 row_min, 並餵入 col_dqs
            if j + 1 >= b {
                let col = j + 1 - b;
                let &(_, mv) = dq.front().unwrap();
                row_min[col] = mv;

                // 更新對應的 col_dq
                let cdq = &mut col_dqs[col];
                while let Some(&(_, v2)) = cdq.back() {
                    if v2 >= mv {
                        cdq.pop_back();
                    } else {
                        break;
                    }
                }
                cdq.push_back((i, mv));
                // pop 過期 row
                if let Some(&(ri, _)) = cdq.front() {
                    if ri + a <= i {
                        cdq.pop_front();
                    }
                }
                // 當高度夠 a 行時，累加最小值
                if i + 1 >= a {
                    ans += cdq.front().unwrap().1;
                }
            }
        }
    }

    // 輸出結果
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    writeln!(out, "{}", ans).unwrap();
}
