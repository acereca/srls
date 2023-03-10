use std::{env, fs, io::Read};
use regex::Regex;
use states::Tokenizer;

mod states;
fn main() {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"((?P<comment>;)|(?P<word>\b\w+\b))").unwrap();

    let mut file = fs::File::open(args[1].as_str()).expect("msg");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("msg");

    let mut tokenizer = Tokenizer::new();
    tokenizer.read_in(data);

}
