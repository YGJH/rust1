use std::env;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::sync::Arc;

use memmap2::Mmap;
use rayon::prelude::*;
use zip::read::ZipArchive;

fn main() {
    // 1. 解析指令列參數
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <zip-file>", args[0]);
        std::process::exit(1);
    }
    let zip_path = &args[1];

    // 2. 使用 Mmap 將整個 ZIP 讀入記憶體
    let file = File::open(zip_path).expect("Cannot open ZIP file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap ZIP file") };
    let arc_mmap = Arc::new(mmap);

    // 3. 先打開一次以獲得檔案數量，之後拆分至各執行緒
    let mut archive = ZipArchive::new(Cursor::new(&*arc_mmap))
        .expect("Failed to read ZIP archive");
    let num_entries = archive.len();
    drop(archive); // 關閉初始檔案

    // 4. 使用 Rayon 進行並行解壓
    (0..num_entries).into_par_iter().for_each(|i| {
        // 每個執行緒都重新打開 ZipArchive
        let mut archive = ZipArchive::new(Cursor::new(&*arc_mmap))
            .expect("Failed to reopen ZIP archive");
        let mut entry = archive.by_index(i).expect("Failed to access entry");

        // 安全處理檔名
        let outpath = match entry.enclosed_name() {
            Some(path) => Path::new(path).to_owned(),
            None => return,
        };

        // 如果是資料夾就創建
        if entry.is_dir() {
            fs::create_dir_all(&outpath).expect("Failed to create directory");
        } else {
            // 確保父目錄存在
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directory");
            }
            // 建立並寫入檔案
            let mut outfile = File::create(&outpath).expect("Failed to create output file");
            std::io::copy(&mut entry, &mut outfile).expect("Failed to write file");
        }
    });

    println!("Finished extracting {} entries from {}", num_entries, zip_path);
}
