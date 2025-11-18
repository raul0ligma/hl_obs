#[allow(unused_imports)]
use ctor as _;
#[allow(unused_imports)]
use dashmap as _;
use libc::{O_CREAT, O_TRUNC, O_WRONLY, c_int, c_void, close, open, write};
#[allow(unused_imports)]
use once_cell as _;
#[allow(unused_imports)]
use redhook as _;
use std::env;
use std::ffi::CString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

fn main() {
    let base_dir = PathBuf::from(env::var("HOME").unwrap_or_default()).join("hl").join("data");
    println!("Base dir: {}", base_dir.display());
    println!("Creating folder structure and exercising open → write → close");

    let date_part = "20251116";
    let base_dirs = ["node_order_statuses_by_block", "node_raw_book_diffs_by_block", "node_fills_by_block"];

    for base in &base_dirs {
        ensure_dir(base_dir.join(base).join("hourly").join(date_part)).expect("failed to create directory tree");
    }

    // Prepare targets (create '9' file in each tree and write a sample line)
    let targets = [
        (
            base_dir.join("node_order_statuses_by_block").join("hourly").join(date_part).join("9"),
            SAMPLE_STATUS_LINE.as_bytes(),
        ),
        (
            base_dir.join("node_raw_book_diffs_by_block").join("hourly").join(date_part).join("9"),
            SAMPLE_DIFF_LINE.as_bytes(),
        ),
        (base_dir.join("node_fills_by_block").join("hourly").join(date_part).join("9"), SAMPLE_FILLS_LINE_1.as_bytes()),
    ];

    for (path, payload) in targets {
        println!("Opening {}", path.display());
        let fd = open_file(&path).expect("open failed");
        write_all(fd, payload).expect("write_all failed");
        write_all(fd, b"\n").expect("newline write failed");
        close_file(fd).expect("close failed");
        println!("Wrote 1 line to {}", path.display());
    }

    // Cleanup: remove created files and prune empty directories if empty
    cleanup(date_part).ok();

    println!("All done.");
}

fn ensure_dir(path: PathBuf) -> io::Result<()> {
    fs::create_dir_all(&path)?;
    Ok(())
}

fn open_file(path: &Path) -> io::Result<c_int> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let c_path = CString::new(path.as_os_str().as_bytes()).expect("CString path");
    let mode = 0o644;
    let flags = O_CREAT | O_WRONLY | O_TRUNC;
    let fd = unsafe { open(c_path.as_ptr(), flags, mode) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(fd)
}

fn write_all(fd: c_int, buf: &[u8]) -> io::Result<()> {
    let mut written = 0usize;
    while written < buf.len() {
        let ptr = unsafe { buf.as_ptr().add(written) } as *const c_void;
        let n = unsafe { write(fd, ptr, buf.len() - written) };
        if n < 0 {
            return Err(io::Error::last_os_error());
        }
        written += n as usize;
    }
    Ok(())
}

fn close_file(fd: c_int) -> io::Result<()> {
    let rc = unsafe { close(fd) };
    if rc < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn cleanup(date_part: &str) -> io::Result<()> {
    let base_dir = PathBuf::from(env::var("HOME").unwrap_or_default()).join("hl").join("data");
    let targets = [
        base_dir.join("node_order_statuses_by_block").join("hourly").join(date_part).join("9"),
        base_dir.join("node_raw_book_diffs_by_block").join("hourly").join(date_part).join("9"),
        base_dir.join("node_fills_by_block").join("hourly").join(date_part).join("9"),
    ];
    for t in &targets {
        drop(fs::remove_file(t));
    }
    let bases = ["node_order_statuses_by_block", "node_raw_book_diffs_by_block", "node_fills_by_block"];
    for base in bases {
        let date_dir = base_dir.join(base).join("hourly").join(date_part);
        let hourly_dir = base_dir.join(base).join("hourly");
        let base_root = base_dir.join(base);
        drop(fs::remove_dir(&date_dir));
        drop(fs::remove_dir(&hourly_dir));
        drop(fs::remove_dir(&base_root));
    }
    Ok(())
}

// Sample payloads (single-line JSON) to write into the files
const SAMPLE_STATUS_LINE: &str = r#"{"local_time":"2025-11-16T09:03:31.951669810","block_time":"2025-11-16T08:45:47.219983285","block_number":463290001,"events":[{"time":"2025-11-16T08:45:47.219983285","user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","hash":"0x1b0550469f5932c21c7f041b9d3e91000040682c3a5c5194becdfb995e5d0cac","builder":null,"status":"open","order":{"coin":"WCT","side":"B","limitPx":"0.14276","sz":"5929.0","oid":43214347550,"timestamp":1763282747219,"triggerCondition":"N/A","isTrigger":false,"triggerPx":"0.0","children":[],"isPositionTpsl":false,"reduceOnly":false,"orderType":"Limit","origSz":"5929.0","tif":"Alo","cloid":null}}]}"#;

const SAMPLE_DIFF_LINE: &str = r#"{"local_time":"2025-11-16T09:03:31.948114729","block_time":"2025-11-16T08:45:47.219983285","block_number":463290001,"events":[{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","oid":43214347550,"coin":"WCT","side":"B","px":"0.14276","raw_book_diff":{"new":{"sz":"5929.0"}}}]}"#;

const SAMPLE_FILLS_LINE_1: &str = r#"{"local_time":"2025-11-16T09:03:31.946384110","block_time":"2025-11-16T08:45:47.219983285","block_number":463290001,"events":[]}"#;
