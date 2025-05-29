use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    println!("請輸入多行文字，輸入空行結束：");

    // 建立一個 Vector 儲存使用者輸入的多行文字
    let stdin = io::stdin();
    let mut lines = Vec::new();

    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            break;
        }
        lines.push(line);
    }

    // 寫入到檔案中，這裡以 "multiline.txt" 為例
    let mut file = File::create("multiline.txt")?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    println!("寫入完成，檔案內容存入 multiline.txt");
   

    let readfile = File::open("multiline.txt")?;
    let reader = BufReader::new(readfile);
    println!("next lines is txt string!");
    for line_result in reader.lines(){
        let line = line_result?;
        println!("{}", line);
    }
    Ok(())

}