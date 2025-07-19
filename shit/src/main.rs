use std::io::{self , Read, Write, BufWriter};

fn divide<U, V>(up: U, down: V) -> Result<f64, String>
where
    U: Into<f64>,
    V: Into<f64>,
{
    let up = up.into();
    let down = down.into();
    if down == 0.0 {
        Err("Divide by zero".to_string())
    } else {
        Ok(up / down)
    }
}


fn main() {
    
    let mut t = String::new();
    let mut writer = BufWriter::with_capacity(100*1024 , io::stdout());
    io::stdin().read_to_string(&mut t).unwrap();
    let mut t = t.split_whitespace();
    let up :f64 = t.next().unwrap().parse().unwrap();
    let down:f64 = t.next().unwrap().parse().unwrap();
    let result = divide(up, down);
    match result {
        Ok(value) => { write!(writer, "the result = {} ", value).unwrap(); },
        Err(_)   => { write!(writer, "divide by zero!!").unwrap(); }
    }



}

