#![allow(unused_imports)]
use std::{io::{self, BufRead, BufReader, BufWriter, Read, Write}, usize};
mod dinic;

use crate::dinic::Dinic;
fn main() {
    let mut buf = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let (n, m): (usize, usize) = (
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().parse().unwrap(),
    );
    let mut d = Dinic::new(n + m + 3);
    
    for i in 0..n {
        reader.read_line(&mut buf).unwrap();

        it = buf.split_whitespace();
        
    }




    let mut writer = BufWriter::new(std::io::stdout());
    write!(writer , "max flow: {}" , d.max_flow(0 , n-1)).unwrap();
    
}
