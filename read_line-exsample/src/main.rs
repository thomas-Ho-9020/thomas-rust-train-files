use std::io::{self, Write};

fn Read_line_get_input(read_cmd: &str) -> String {
    print!("{}", read_cmd);
    io::stdout().flush().expect("failed");
    let mut butter = String::new();
    io::stdin().read_line(&mut butter).expect("failed");
    butter.trim().to_string()
}

fn main() {
    let message = Read_line_get_input("plz input your message: ");
    println!("{}",message);
}
 