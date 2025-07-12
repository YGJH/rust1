struct Solver {
    n: i32,
}

impl Solver {
    // 構造函數
    fn new() -> Self {
        let n = 300;
        let solver = Solver { n };
        println!("{} add(n): {}", n, solver.add(n));
        solver
    }

    // 方法
    fn add(&self, n: i32) -> i32 {
        n + n
    }
}

// 如果需要解構時的清理，可以實現 Drop trait
impl Drop for Solver {
    fn drop(&mut self) {
        // 解構時執行的程式碼（如果需要的話）
        // 在這個例子中不需要特別處理
    }
}

fn main() {
    let _solve = Solver::new();
}