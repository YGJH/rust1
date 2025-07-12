#![allow(dead_code, unused_imports, unused_macros)]

use std::io::{Write , Read, BufWriter};


fn main() {
    let mut buf = String::new();
    let mut writer: BufWriter<std::io::Stdout> = BufWriter::with_capacity(100*1024 , std::io::stdout());
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_ascii_whitespace();
    let mut t: i32 = it.next().unwrap().parse().unwrap();
    while t > 0 {
        t -= 1;
        let n:usize = it.next().unwrap().parse().unwrap();
        let arr: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
        let mut ans: Vec<u8> = vec![b'0'; n];
        let mut cur_min = i32::MAX;
        for i in 0..n {
            let now: i32 = arr[i]; 
            if cur_min > now {
                ans[i] = b'1';
                cur_min = now;
            }
        }
        let mut cur_max = i32::MIN;
        for i in (0..n).rev() {
            let now = arr[i];
            if cur_max < now {
                ans[i] = b'1';
                cur_max = now;
            }
        }
        writer.write_all(&ans).unwrap();
        writer.write_all(b"\n").unwrap();
    
    }
    writer.flush().unwrap();

}