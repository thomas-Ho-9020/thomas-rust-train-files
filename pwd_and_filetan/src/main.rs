use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

/// 遞迴複製來源目錄到目的目錄的函數
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    // 若目的目錄不存在就創建它
    if !dst.exists() {
        fs::create_dir_all(dst)
            .expect(&format!("無法建立目的目錄: {:?}", dst));
    }

    // 讀取來源目錄的所有項目
    for entry in fs::read_dir(src)
        .expect(&format!("無法讀取來源目錄: {:?}", src))
    {
        let entry = entry.expect("無法取得目錄項目");
        let entry_path = entry.path();
        // 建立對應於目的目錄下的路徑
        let dest_path = dst.join(entry.file_name());

        // 如果是目錄則遞迴複製，否則直接複製檔案
        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)
                .expect(&format!("無法從 {:?} 複製到 {:?}", entry_path, dest_path));
        }
    }

    Ok(())
}

/// 取得使用者輸入，並回傳去除換行字元後的字串
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("無法 flush 輸出");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("無法讀取輸入");
    buffer.trim().to_string()
}

fn main() -> io::Result<()> {
    // 取得目前工作目錄（類似 pwd 命令）
    let current_dir = std::env::current_dir()
        .expect("無法取得目前工作目錄");

    let dst_location = Path::new(r"C:\copy");
    //println!("目前工作目錄: {}", current_dir.display());

    // 要求使用者輸入來源和目的的相對路徑或絕對路徑
    let src_input = get_input("請輸入來源目錄路徑: ");
    let dst_input = get_input("請輸入目的目錄名稱（相對於目前目錄）: ");

    // 若輸入為絕對路徑就直接使用，否則與目前目錄結合
    let src_path = {
        let path = PathBuf::from(&src_input);
        if path.is_absolute() {
            path
        } else {
            current_dir.join(path)
        }
    };

    let dst_path = dst_location.join(&dst_input);

    // 檢查來源路徑是否存在
    if !src_path.exists() {
        eprintln!("來源路徑不存在: {}", src_path.display());
        return Ok(());
    }

    // 檢查來源是否為目錄
    if !src_path.is_dir() {
        eprintln!("來源路徑不是一個目錄: {}", src_path.display());
        return Ok(());
    }

    // 執行遞迴複製
    match copy_dir_recursive(&src_path, &dst_path) {
        Ok(_) => println!("目錄複製成功!"),
        Err(e) => eprintln!("複製過程中發生錯誤: {}", e),
    }

    Ok(())
}