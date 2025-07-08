use std::{
    fs::{self,File},
};

fn main() -> std::io::Result <()> {
    fs::remove_dir_all("try")?;
    fs::create_dir_all("try/.kk")?;
    File::create("try/.kk/test.pdf")?;
    fs::copy("try/.kk/test.pdf","test.txt")?;
    //fs::remove_dir_all("try")?;
    Ok(())
}