use std::env;
use std::fs;
use std::fs::File;
use std::io::{Cursor, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use memmap2::Mmap;
use rayon::prelude::*;
use zip::read::ZipArchive;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // 1. 解析參數：ZIP 檔 & 可選輸出目錄
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <zip-file> [output-dir]", args[0]);
        std::process::exit(1);
    }
    let zip_path = Path::new(&args[1]);
    let out_dir = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        env::current_dir().expect("Cannot get current directory")
    };
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // 2. mmap 整檔
    let file = File::open(&zip_path).expect("Cannot open ZIP file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap ZIP file") };
    let arc_mmap = Arc::new(mmap);

    // 3. 獲取條目數量
    let mut archive = ZipArchive::new(Cursor::new(&*arc_mmap))
        .expect("Failed to read ZIP archive");
    let num = archive.len();
    drop(archive);

    // 4. 準備並行索引
    let indices: Vec<usize> = (0..num).collect();

    // 5. 進度條
    let pb = ProgressBar::new(num as u64);
    pb.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} [{elapsed_precise}]")
            .unwrap()
            .progress_chars("##-")
    );

    // 6. 並行解壓
    indices.into_par_iter().for_each(|i| {
        let mut archive = ZipArchive::new(Cursor::new(&*arc_mmap))
            .expect("Failed to reopen archive");
        let mut entry = archive.by_index(i).expect("Failed to access entry");

        let name = match entry.enclosed_name() {
            Some(n) => n.to_owned(),
            None => { pb.inc(1); return; }
        };
        let path = out_dir.join(name);

        if entry.is_dir() {
            fs::create_dir_all(&path).expect("Failed to create directory");
        } else {
            if let Some(p) = path.parent() {
                fs::create_dir_all(p).expect("Failed to create parent dir");
            }
            let f = File::create(&path).expect("Failed to create file");
            let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, f);
            std::io::copy(&mut entry, &mut writer).expect("Write error");
            writer.flush().unwrap();
        }
        pb.inc(1);
    });

    pb.finish_with_message(format!("Done: {} entries extracted to {}", num, out_dir.display()));
}

// 請在 Cargo.toml 中加入：
// memmap2 = "0.5"
// rayon = "1.7"
// zip = "0.6"
// indicatif = "0.17"
