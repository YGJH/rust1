use std::env;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use memmap2::Mmap;
use rayon::prelude::*;
use zip::read::ZipArchive;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // 1. 解析指令列參數：ZIP 檔路徑，及可選的輸出目錄
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <zip-file> [output-dir]", args[0]);
        std::process::exit(1);
    }
    let zip_path = Path::new(&args[1]);
    // 若有指定輸出目錄，否則使用目前工作目錄
    let out_dir = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        env::current_dir().expect("Cannot get current directory")
    };
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // 2. Memory-map ZIP 檔以加快多次讀取
    let file = File::open(&zip_path).expect("Cannot open ZIP file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap ZIP file") };
    let arc_mmap = Arc::new(mmap);

    // 3. 先讀取一次以獲得條目數量
    let mut archive = ZipArchive::new(Cursor::new(&*arc_mmap))
        .expect("Failed to read ZIP archive");
    let num_entries = archive.len();
    drop(archive);

    // 4. 建立進度條
    let pb = ProgressBar::new(num_entries as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{bar:40.cyan/blue} {pos}/{len} [{elapsed_precise}]"
        )
        .unwrap()
        .progress_chars("##-"),
    );

    // 5. 並行解壓每個條目
    (0..num_entries).into_par_iter().for_each(|i| {
        let mut archive = ZipArchive::new(Cursor::new(&*arc_mmap))
            .expect("Failed to reopen ZIP archive");
        let mut entry = archive.by_index(i).expect("Failed to access entry");

        // 安全獲取檔名
        let entry_name = match entry.enclosed_name() {
            Some(path) => path.to_owned(),
            None => { pb.inc(1); return; }
        };
        let outpath = out_dir.join(entry_name);

        if entry.is_dir() {
            fs::create_dir_all(&outpath).expect("Failed to create directory");
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directory");
            }
            let mut outfile = File::create(&outpath).expect("Failed to create file");
            std::io::copy(&mut entry, &mut outfile).expect("Failed to write file");
        }
        pb.inc(1);
    });

    pb.finish_with_message(&format!(
        "Extracted {} entries to {}",
        num_entries,
        out_dir.display()
    ));
}

// 請在 Cargo.toml 中加入：
// memmap2 = "0.5"
// rayon = "1.7"
// zip = "0.6"
// indicatif = "0.17"
