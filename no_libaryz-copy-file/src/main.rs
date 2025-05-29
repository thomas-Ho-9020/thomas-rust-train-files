use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let source = r"C:\Users\ThomasHo\Desktop\thomas-rust-file-manager-tut-and-training\trian\multilinewrite\multiline.txt";
    let destination = "multiline_copy_manual.txt";
    
    // 開啟來源檔案
    let mut src_file = File::open(source)?;
    let mut contents = Vec::new();
    src_file.read_to_end(&mut contents)?;

    // 建立目的檔案並寫入內容
    let mut dest_file = File::create(destination)?;
    dest_file.write_all(&contents)?;
    
    println!("手動複製完成，檔案存放在 {}", destination);
    Ok(())
}