use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};


fn main() -> io::Result<()>{
    //1.create and write a txt
    //let mut file = File::create("i_am_thomas.txt")?;
    //writeln!(file, "hi i am thomas is learn rust boy")?;
    //writeln!(file, "tmr have more train hlep me !!!")?;

    //2.open file and read
    let readfile = File::open("i_am_thomas.txt")?;
    let reader = BufReader::new(readfile);
    println!("next lines is txt string!");
    for line_result in reader.lines(){
        let line = line_result?;
        println!("{}", line);
    }
    Ok(())
}
