use std::fs::File;
use std::io::Write;

fn main() {
    println!("Opening test files...");

    let mut f1 =
        File::create("/tmp/test/node_order_statuses_by_block/test.txt").expect("Failed to create statuses file");
    f1.write_all(b"status\n").ok();
    println!("Created statuses file");

    let mut f2 = File::create("/tmp/test/node_raw_book_diffs_by_block/test.txt").expect("Failed to create diffs file");
    f2.write_all(b"diff\n").ok();
    println!("Created diffs file");

    let mut f3 = File::create("/tmp/test/node_fills_by_block/test.txt").expect("Failed to create fills file");
    f3.write_all(b"fill\n").ok();
    println!("Created fills file");

    println!("Done!");
}
