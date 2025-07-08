use std::{
    fs::{self,File},
    path::Path,
};

fn copy_dir_recursive(src: &Path,dst: &Path) {
    if !dst.exists() {
        fs::create_dir_all(dst).expect(&format! ("Failed to create: {:?}",dst)); 
    }
    for entry_dir in fs::read_dir(src).expect(&format! ("Failed to read: {:?}",src)) {
        let entry_dir = entry_dir.expect("Failed to entry");
        let entry_dir_path = entry_dir.path();
        let dest_path = dst.join(entry_dir.file_name());

        if entry_dir_path.is_dir() {
            copy_dir_recursive(&entry_dir_path, &dest_path);

        } else {
            fs::copy(&entry_dir_path, &dest_path).expect(&format! ("Failed to copy files from {:?} to {:?}",entry_dir_path, dest_path));

        }
    }
}

fn main() {
    fs::create_dir_all("test/src").expect("create dir error!!!");
    File::create("test/src/thomas.txt").expect("create file errar!!");
    let src = Path::new("test/src");
    let dst = Path::new("copy/dst");
    copy_dir_recursive(src, dst);
    fs::remove_dir_all("test").expect("not remove!");
    println!("OK");

}