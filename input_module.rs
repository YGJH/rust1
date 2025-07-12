use std::{collections::VecDeque, io::Read};
 
struct Input {
    inner: VecDeque<String>,
}
 
impl Input {
    fn new() -> Self {
        
        let mut inner = String::new();
        std::io::stdin().read_to_string(&mut inner).unwrap();
        Input {
            inner: inner
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.inner.pop_front().unwrap().parse().unwrap()
    }
}
