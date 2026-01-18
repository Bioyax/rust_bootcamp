use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "Read and write binary files in hexadecimal", long_about = None)]
struct Args {
    /// Target file
    #[arg(short, long = "file", value_name = "FILE", required = true)]
    file: PathBuf,

    /// Read mode (display hex)
    #[arg(short, long, conflicts_with = "write")]
    read: bool,

    /// Write mode (hex string to write)
    #[arg(short, long, value_name = "HEX")]
    write: Option<String>,

    /// Offset in bytes (decimal or 0x hex)
    #[arg(short = 'o', long = "offset", value_name = "OFF", default_value = "0")]
    offset: String,

    /// Number of bytes to read
    #[arg(short, long = "size", value_name = "N")]
    size: Option<usize>,
}

fn parse_offset(offset_str: &str) -> Result<u64, std::num::ParseIntError> {
    if let Some(stripped) = offset_str.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else {
        offset_str.parse::<u64>()
    }
}

fn handle_write(file_path: PathBuf, offset: u64, hex_string: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
    file.seek(SeekFrom::Start(offset))?;

    match hex::decode(hex_string) {
        Ok(bytes) => {
            file.write_all(&bytes)?;
            println!("Successfully written {} bytes.", bytes.len());
            Ok(())
        }
        Err(e) => {
            eprintln!("Error decoding hex string: {}", e);
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        }
    }
}

fn handle_read(file_path: PathBuf, offset: u64, size: Option<usize>) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(offset))?;

    let mut buffer = Vec::new();
    let bytes_read = if let Some(s) = size {
        buffer.resize(s, 0);
        file.read(&mut buffer)?
    } else {
        file.read_to_end(&mut buffer)?
    };
    buffer.truncate(bytes_read);


    if let Ok(s) = std::str::from_utf8(&buffer) {
        println!("{}", s);
    } else {
        // Basic hexdump format
        for (i, chunk) in buffer.chunks(16).enumerate() {
            print!("{:08x}: ", offset as usize + i * 16);
            for byte in chunk {
                print!("{:02x} ", byte);
            }
            // Add padding for the last line
            if chunk.len() < 16 {
                for _ in 0..(16 - chunk.len()) {
                    print!("   ");
                }
            }
            print!("|");
            for &byte in chunk {
                if (32..=126).contains(&byte) {
                    print!("{}", byte as char);
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let offset = match parse_offset(&args.offset) {
        Ok(off) => off,
        Err(e) => {
            eprintln!("Error: Invalid offset value: {}", e);
            std::process::exit(1);
        }
    };

    if args.read {
        if let Err(e) = handle_read(args.file, offset, args.size) {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    } else if let Some(hex_string) = args.write {
        if let Err(e) = handle_write(args.file, offset, hex_string) {
            eprintln!("Error writing to file: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: You must specify either --read or --write mode.");
        std::process::exit(1);
    }
}