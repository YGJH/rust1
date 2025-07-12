use std::io::{Write, BufWriter , BufReader , Read};
use std::collections::BinaryHeap;


fn main() {
    let mut reader = BufReader::with_capacity(1024 * 20, io::stdin());
    let t = String::new();
    reader.read_line(&mut t);
    let mut bh :BinaryHeap<(i32 , i32)> = BinaryHeap::new();
    while reader.read_line(&mut t) {
        let mut it: std::str::SplitAsciiWhitespace<'_> = t.split_ascii_whitespace();
        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        reader.read_line(&mut t);
        t.split_ascii_whitespace();
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        let k = it.next().unwrap().parse().unwrap();
        let mut Edge = vec![Vec::new(): n];
        let _x : i32;
        let _y : i32;
        let _w : i32;
        for i in 0..m {
            reader.read_line(&mut t);
            t.split_ascii_whitespace();
            _x = it.next().unwrap().parse().unwrap();
            _y = it.next().unwrap().parse().unwrap();
            _w = it.next().unwrap().parse().unwrap();
            Edge[_x].push_back((_y , _w));
            if _x == x {
                bh.push(x)
            }
        }
    }

    Ok(());
}