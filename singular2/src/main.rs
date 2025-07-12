    use std::io::{self, BufWriter, Read, Write};
    use std::collections::VecDeque;
    // mod ac;


    // #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    // struct Pair<T1, T2> {
    //     first: T1,
    //     second: T2,
    // }

    fn main() {
        // let mut reader = BufReader::new(io::stdin());
        let mut writer: BufWriter<io::Stdout> = BufWriter::new(io::stdout());
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        let mut it: std::str::SplitAsciiWhitespace<'_> = buf.split_ascii_whitespace();

        let n : usize = it.next().unwrap().parse().unwrap();
        let _k : usize = it.next().unwrap().parse().unwrap();
        let _x : usize = it.next().unwrap().parse().unwrap();
        let arr: Vec<i64> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
        let _min: i64 = i64::MIN;
        let mut dp = vec![_min; (n + 2) as usize];
        
        for _i in 0..n.min(_k) {
            dp[_i] = arr[_i] as i64;
        }
        
        for _i in 2..=_x {
            let mut dp_curr: Vec<i64> = vec![_min;  (n+1) as usize];
            let mut dq: VecDeque<(usize , i64)> = VecDeque::new();
            for _j in 0..n {
                while let Some(&(idx, _val)) = dq.front() {
                    if idx + _k < _j { dq.pop_front(); } else { break; }
                }

                if let Some(&(_idx , val)) = dq.front() {
                    dp_curr[_j] = val + arr[_j] as i64;
                }
                if dp[_j] > _min {
                    while let Some(&(_idx , val )) = dq.back() {
                        if val <= dp[_j] { dq.pop_back(); } else { break; }
                    }
                }   
                dq.push_back((_j , dp[_j]));
            }

            dp = dp_curr;
            // write!(writer , "\ndp = ").unwrap();
            // let str: String = dp.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
            // writeln!(writer,"{}", str).unwrap();
        }


        let mut ans = _min;
        for i in (n.saturating_sub(_k))..n {
            ans = ans.max(dp[i]);
        }
        if ans < 0 {
            write!(writer,"-1").unwrap();
        } else {
            write!(writer , "{}" , ans).unwrap();
        }
        


    }

