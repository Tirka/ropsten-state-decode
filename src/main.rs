use std::fs::File;
use rlp::Rlp;

fn main() {
    let state_path = "/home/maksimv/ropsten-state.rlp";
    let state_path = "/home/maksimv/ropsten-state-stripped.rlp";
    // let state_path = "test.rlp";
    let state_file = File::open(state_path).unwrap();

    let mmap = unsafe { memmap::MmapOptions::new().map(&state_file).unwrap() };

    println!("ropsten-state.rlp");

    let rlp = Rlp::new(mmap.as_ref());

    proceed_rlp(0, rlp);
}

fn proceed_rlp(nest_lvl: u32, rlp: Rlp) {
    for _ in 0..nest_lvl {
        print!("    ");
    }

    match rlp.prototype().unwrap() {
        rlp::Prototype::Null => {
            println!("Null");
        },
        rlp::Prototype::Data(len) => {
            println!("Data({len}): {}", hex::encode(rlp.as_val::<Vec<u8>>().unwrap()));
        },
        rlp::Prototype::List(len) => {
            println!("List({len})");
            for (_idx, child) in rlp.into_iter().enumerate() {
                proceed_rlp(nest_lvl + 1, child);
            }
        },
    }
}
