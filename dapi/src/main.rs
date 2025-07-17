#![allow(unused_imports, dead_code)]
use rug::Float;
use f128::f128;
// use num_traits::Float;
// use std::f64;

fn main() {
    // let f = f64::consts::PI / 4.0;

    // acos(cos(pi/4))
    // let abs_difference = (f.cos().acos() - f64::consts::PI / 4.0).abs();

    let a = Float::with_val(1024, 22);
    let b = Float::with_val(1024, 7);
    // let pi = Float::with_val(1024 , );
    let c = a.clone() / b.clone();
    // print 40 decimal places of the rug::Float result
    println!("1 / 3 in 128 bit = {:.40}", c);
    // println!("pi in 128 bit = {:.40}", pi);
    let a = f128::from(1.0f64);
    let b = f128::from(3.0f64);
    let c = a / b;
    println!("1 / 3 = {:.40}", c);
}
