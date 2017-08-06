use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::io::Error;


fn run() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
    run().unwrap();
}