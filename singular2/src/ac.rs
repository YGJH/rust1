use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let x: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();

    // 用一個極小值表示 −∞
    let neg_inf = std::i64::MIN / 2;

    // dp_prev[i] = dp[j−1][i], dp_curr[i] = dp[j][i]
    let mut dp_prev = vec![neg_inf; n];

    // 初始化 j=1：只能選前 k 張中的
    for i in 0..n.min(k) {
        dp_prev[i] = a[i];
    }

    // 從 j=2 到 j=x
    for _j in 2..=x {
        let mut dp_curr = vec![neg_inf; n];
        let mut dq: VecDeque<(usize, i64)> = VecDeque::new();
        // 滑動過程
        for i in 0..n {
            // 先將「窗口左端」之外的值彈出
            while let Some(&(idx, _val)) = dq.front() {
                if idx + k < i { dq.pop_front(); } else { break; }
            }
            // 此時 deque.front() 就是 max(dp_prev[p])，p∈[i−k, i−1]
            if let Some(&(_idx, best)) = dq.front() {
                dp_curr[i] = best + a[i];
            }   
            // 將 dp_prev[i] 插入 deque（保持單調遞減）
            if dp_prev[i] > neg_inf {
                while let Some(&(_idx, val)) = dq.back() {
                    if val <= dp_prev[i] { dq.pop_back(); } else { break; }
                }
                dq.push_back((i, dp_prev[i]));
            }
        }
        dp_prev = dp_curr;
    }

    // 答案是在最後 x 張選擇中，必須覆蓋最後一段 [n−k, n−1]
    let mut ans = neg_inf;
    for i in (n.saturating_sub(k))..n {
        ans = ans.max(dp_prev[i]);
    }

    if ans < 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}