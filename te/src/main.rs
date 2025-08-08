use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

fn fun() -> Result<&'static str, Box<dyn std::error::Error>> {
    Ok("fjiwoejqf")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("test.txt")?;
    let mut reader = BufReader::new(file);

    for (idx, i) in reader.lines().enumerate() {
        let l = i?;
        println!("idx=  {} l = {}", idx, l);
    }

    let meta = fs::metadata("test.txt")?;
    println!("大小：{} bytes", meta.len());
    println!("是否為目錄？{}", meta.is_dir());
    Ok(())
}
