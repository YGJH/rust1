#![allow(unused_imports)]
use std::fs::File;
use std::fs;
use std::io::{self, Read , Write , BufWriter};

fn main() ->  Result<(), Box<dyn std::error::Error>>  {
    let mut file = File::create("test.txt")?;
    let mut writer = BufWriter::new(file);
    write!(writer , "幹你娘老")?;
    writer.flush()?;
    // let meta = fs::metadata("test.txt");
    // println!("大小：{} bytes", meta.len());
    // println!("是否為目錄？{}", meta?.is_dir());

    // // 複製、重新命名、刪除
    // fs::copy("output.txt", "backup.txt");
    // fs::rename("backup.txt", "archive/backup.txt");
    // fs::remove_file("archive/backup.txt");

    Ok(())
}
