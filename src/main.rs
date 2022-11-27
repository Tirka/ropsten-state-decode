use clap::Parser;
use rlp::Rlp;

/// Simple program to parse Geth snapshots
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Input RLP snapshot file path
   #[arg(short, long, value_name = "FILE")]
   snapshot: String,
}

fn main() {
    let args = Args::parse();

    let snapshot_full_path = std::fs::canonicalize(&args.snapshot)
        .expect("Can't get absolute file path of snapshot");
    
    let snapshot_file_name = snapshot_full_path.iter().last()
        .expect("Can't parse absolute path")
        .to_string_lossy();

    let snapshot = std::fs::File::open(&snapshot_full_path)
        .unwrap_or_else(|_| panic!("Can't open snapshot file: '{}'", &args.snapshot));

    let snapshot = unsafe { memmap::MmapOptions::new().map(&snapshot).unwrap() };

    println!("File: {snapshot_file_name}");

    let rlp = Rlp::new(&snapshot);

    proceed_rlp(0, rlp);
}

fn proceed_rlp(nest_lvl: u32, rlp: Rlp) {
    print_padding(nest_lvl);

    match rlp.prototype().unwrap() {
        rlp::Prototype::Null => {
            println!("<NULL>");
        },
        rlp::Prototype::Data(len) => {
            let data: Vec<u8> = rlp.as_val().unwrap();
            let hex = if data.is_empty() { "0".to_string() } else { hex::encode(&data) };
            let as_string = std::str::from_utf8(&data).map(|s| s.trim())
                .unwrap_or("<N/A>");
            println!("Data({len}): 0x{} [{}]", hex, as_string);
        },
        rlp::Prototype::List(len) => {
            println!("List({len}):");
            for (_, child) in rlp.into_iter().enumerate() {
                proceed_rlp(nest_lvl + 1, child);
            }
        },
    }
}

fn print_padding(nest_lvl: u32) {
    for _ in 0..nest_lvl {
        print!("    ");
    }
}