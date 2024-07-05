use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn hex_to_binary(input_file: &str, output_file: &str) -> io::Result<()> {
    let input_path = Path::new(input_file);
    let output_path = Path::new(output_file);

    let input_file = File::open(&input_path)?;
    let mut output_file = File::create(&output_path)?;

    for line in io::BufReader::new(input_file).lines() {
        let line = line?;
        let hex_str = line.split('|').next().unwrap().trim().replace(" ", "");
        let hex_bytes = hex::decode(hex_str).expect("Invalid hex data");
        output_file.write_all(&hex_bytes)?;
    }

    Ok(())
}

fn main() {
    let input_file = "hexdump.txt";
    let output_file = "binary_output.txt";

    match hex_to_binary(input_file, output_file) {
        Ok(_) => println!("Binary file '{}' created from hexdump.", output_file),
        Err(e) => eprintln!("Error: {}", e),
    }
}

