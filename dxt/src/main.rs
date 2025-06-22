use std::{env, fs, path::PathBuf, sync::Arc};
use fs::File;
use memmap2::Mmap;
use rayon::prelude::*;
use zip::read::ZipArchive;
use zip::CompressionMethod;
use indicatif::{ProgressBar, ProgressStyle};
use crossbeam_channel::unbounded;
use miniz_oxide::inflate::decompress_to_vec_zlib;

/// 儲存每個條目的元資料
struct EntryInfo {
    name: PathBuf,
    is_dir: bool,
    data_start: usize,
    compressed_size: usize,
    compression: CompressionMethod,
}

fn main() {
    // 1. 解析參數：ZIP 檔 & 可選輸出目錄
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <zip-file> [output-dir]", args[0]);
        std::process::exit(1);
    }
    let zip_path = &args[1];
    let out_dir = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        env::current_dir().expect("Cannot get current directory")
    };
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // 2. mmap 整檔並解析條目元資料一次
    let file = File::open(zip_path).expect("Cannot open ZIP file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap ZIP file") };
    let arc_mmap = Arc::new(mmap);
    let cursor = std::io::Cursor::new(&*arc_mmap);
    let mut archive = ZipArchive::new(cursor).expect("Failed to read ZIP archive");

    let mut entries = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        let f = archive.by_index(i).expect("Invalid index");
        if let Some(p) = f.enclosed_name() {
            entries.push(EntryInfo {
                name: p.to_owned(),
                is_dir: f.is_dir(),
                data_start: f.data_start() as usize,
                compressed_size: f.compressed_size() as usize,
                compression: f.compression(),
            });
        }
    }
    let total = entries.len();

    // 3. 初始化進度條 (僅在主執行緒更新)
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::with_template("{bar:40.green/black} {pos}/{len} [{elapsed_precise}]")
            .unwrap()
            .progress_chars("#>-"),
    );

    // 4. 並行解壓，但主執行緒負責更新進度條
    let out_dir = Arc::new(out_dir);
    let (tx, rx) = unbounded();
    entries.into_par_iter().for_each_with(tx.clone(), |s, entry| {
        let out_dir = Arc::clone(&out_dir);
        if entry.is_dir {
            fs::create_dir_all(out_dir.join(&entry.name)).ok();
        } else {
            let path = out_dir.join(&entry.name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).ok();
            }
            let slice = &arc_mmap[entry.data_start..entry.data_start + entry.compressed_size];
            let data = match entry.compression {
                CompressionMethod::Deflated => {
                    // 使用 miniz_oxide 快速解壓
                    decompress_to_vec_zlib(slice).expect("Decompression failed")
                }
                CompressionMethod::Stored => slice.to_vec(),
                _ => Vec::new(),
            };
            fs::write(&path, &data).expect("Failed to write file");
        }
        s.send(()).unwrap();
    });
    drop(tx);

    // 主執行緒接收訊號並更新進度條
    for _ in 0..total {
        rx.recv().unwrap();
        pb.inc(1);
    }
    pb.finish_with_message(format!("Extracted {} entries to {:?}", total, out_dir));
}

// 請在 Cargo.toml 中加入：
// memmap2 = "0.5"
// rayon = "1.7"
// zip = "0.6"
// indicatif = "0.17"
// crossbeam-channel = "0.5"
// miniz-oxide = "0.5"
