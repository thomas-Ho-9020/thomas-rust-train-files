use std::fs;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("some")?;
    //fs::remove_dir_all("some")?;
    Ok(())
}