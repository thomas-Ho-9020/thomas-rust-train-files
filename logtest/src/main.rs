use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use chrono::Local;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    
    println!("請輸入日誌內容，若要退出請輸入 'exit'：");

    // 進入無限讀取迴圈
    loop {
        // 提示輸入
        print!("> ");
        // 由於 print! 不會自動 flush，需要手動 flush 輸出緩衝區
        io::stdout().flush()?;
        
        let mut input = String::new();
        // 從標準輸入讀取一行
        stdin.lock().read_line(&mut input)?;
        // 移除頭尾的空白（含換行）
        let input = input.trim();
        
        // 判斷是否是退出指令
        if input.eq_ignore_ascii_case("exit") {
            println!("程式退出。");
            break;
        }

        // 取得當前的本地時間
        let now = Local::now();
        // 格式化為 yyyy-MM-dd HH:mm:ss 字串
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        // 組成日誌紀錄訊息，每筆紀錄前面加上時間戳
        let log_entry = format!("{} - {}\n", timestamp, input);

        // 以 append 模式開啟或建立 log.txt 檔案
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")?;
        
        // 將日誌紀錄寫入檔案
        file.write_all(log_entry.as_bytes())?;
        
        println!("已記錄：{}", log_entry.trim());
    }
    
    Ok(())
}