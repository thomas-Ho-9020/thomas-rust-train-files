use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let source = r"C:\Users\ThomasHo\Desktop\thomas-rust-file-manager-tut-and-training\trian\multilinewrite\multiline.txt";
    let destination = "multiline_copy.txt";
    
    // 使用 fs::copy，會複製來源檔案到目的檔案
    fs::copy(source, destination)?;
    println!("檔案已經複製到 {}", destination);
    
    Ok(())
}