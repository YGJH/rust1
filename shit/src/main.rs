use std::io::{self, BufReader, Read, Write};

fn main() {
    /* ---------- Fast input ---------- */
    let mut input: String = String::new();
    let mut reader: BufReader<io::Stdin> = BufReader::new(io::stdin());
    reader.read_to_string(&mut input).unwrap();

    let mut it: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let s: &[u8] = it.next().unwrap().as_bytes();           // slice 直接用

    /* ---------- dist ---------- */
    const INF: i32 = i32::MAX;
    let mut dist: Vec<i32> = vec![INF; n + 1];

    // 手寫環形佇列
    let mut q: Vec<usize> = vec![0usize; n + 2];
    let (mut head, mut tail) = (0usize, 0usize);

    dist[n] = 0;
    q[tail] = n; tail += 1;

    for i in (0..n).rev() {
        if s[i] == b'1' { continue; }

        // pop_front 仍在視窗外
        while head < tail && q[head] > i + m {
            head += 1;
        }
        if head < tail {
            dist[i] = dist[q[head]] + 1;
        }
        if dist[i] != INF {
            // pop_back ≥ dist[i]
            while head < tail && dist[q[tail - 1]] >= dist[i] {
                tail -= 1;
            }
            q[tail] = i; tail += 1;
        }
    }

    if dist[0] == INF {
        println!("-1");
        return;
    }

    /* ---------- rebuild ---------- */
    let mut ans = Vec::with_capacity(dist[0] as usize);
    let mut pos = 0usize;
    while pos != n {
        // 最多 m 次，理論 O(N)
        let mut step = 1;
        while step <= m {
            let nxt = pos + step;
            if nxt <= n && s[nxt] == b'0' && dist[nxt] == dist[pos] - 1 {
                ans.push(step);
                pos = nxt;
                break;
            }
            step += 1;
        }
    }

    /* ---------- fast output ---------- */
    let mut out = io::BufWriter::new(io::stdout());
    for (i, v) in ans.iter().enumerate() {
        if i + 1 == ans.len() {
            writeln!(out, "{v}").unwrap();
        } else {
            write!(out, "{v} ").unwrap();
        }
    }
}
