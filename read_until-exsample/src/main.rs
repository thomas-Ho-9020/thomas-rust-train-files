use std::io::{self, BufRead, Write};

fn Read_until_get_input(read_cmd: &str) -> String {
    print!("{}", read_cmd);
    io::stdout().flush().expect("failed");

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut butter = Vec::new();
    handle.read_until(b'\n', &mut buffer).expect("failed");

    let input = String::from_utf8_lossy(&butter);
    input.trim().to_string()
}

fn main() {
    let meassage = Read_until_get_input("plz enter your meassage:");
    println!("{}",meassage);
}