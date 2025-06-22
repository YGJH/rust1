use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::io::{self, BufWriter, Read, Write};
use std::collections::VecDeque;
use std::cmp;



#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pair<T1, T2> {
    first: T1,
    second: T2,
}

fn main() {
    // let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();

    let n : usize = it.next().unwrap().parse().unwrap();
    let k : usize = it.next().unwrap().parse().unwrap();
    let _x : usize = it.next().unwrap().parse().unwrap();
    let arr: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
    
    let _min: i64 = i64::MIN;
    let mut dp = vec![_min , (n + 1) as i64];
    
    for _i in 0..n {
        dp[_i] = arr[_i] as i64;
    }
    
    for _i in 0.._x {
        let mut dp_curr: Vec<i64> = vec![_min,  (n+1)as i64];
        let mut dq: VecDeque<(usize , i64)> = VecDeque::new();
        for _j in 0..n {
            while let Some(&(idx, val)) = dq.front() {
                if idx + _i < _i { dq.pop_front(); } else { break; }
            }

            if let Some(&(idx , val)) = dq.front() {
                dp_curr[_i] = val + val;
            }

            while let Some(&(idx , val )) = dq.back() {
                if val < dp_curr[_i] { dq.pop_back(); } else { break; }
            }
            dq.push_back((_i , dp_curr[_i]));
        }

        dp = dp_curr;
        write!(writer , "\ndp = ").unwrap();
        let str: String = dp.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
        write!(writer,"{}", str).unwrap();
    }





}

