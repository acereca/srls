use std::{env, fs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = fs::File::open(args[1].as_str()).expect("msg");
    let mut data = String::new();

}
