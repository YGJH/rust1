use std::time::Instant;
extern crate rand;
use rand::Rng;

fn main() {
    run_performance_benchmark();
    println!("");
    demonstrate_stability();
}

fn run_performance_benchmark() {
    println!("--- Performance Benchmark ---");
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..1_000_000).map(|_| rng.gen_range(0..1_000_000)).collect();

    // Benchmark unstable_sort
    let mut unstable_sorted = nums.clone();
    let start = Instant::now();
    unstable_sorted.sort_unstable();
    let duration = start.elapsed();
    println!("Time to sort (unstable): {:?}", duration);

    // Benchmark stable_sort
    let mut stable_sorted = nums.clone();
    let start = Instant::now();
    stable_sorted.sort();
    let duration = start.elapsed();
    println!("Time to sort (stable):   {:?}", duration);
}

fn demonstrate_stability() {
    println!("--- Stability Demonstration ---");

    // Create a vector of tuples (key, original_order)
    let mut pairs = vec![(2, "a"), (1, "b"), (2, "c"), (1, "d"), (3, "e")];

    println!("Original pairs: {:?}", pairs);

    // Sort with unstable_sort
    let mut unstable_sorted_pairs = pairs.clone();
    unstable_sorted_pairs.sort_unstable_by_key(|k| k.0);
    println!("After unstable sort: {:?}", unstable_sorted_pairs);

    // Sort with stable_sort
    let mut stable_sorted_pairs = pairs.clone();
    stable_sorted_pairs.sort_by_key(|k| k.0);
    println!("After stable sort:   {:?}", stable_sorted_pairs);
}