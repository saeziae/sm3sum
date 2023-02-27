// sm3sum.rs
//
// Computes the SM3 hash of one or more files, or standard input, and prints the result.
//
// License: MIT License
//
// Author: Estela ad Astra
//

extern crate sm3;
use std::env;
use std::fs::File;
use std::io::{self, Read, Result};
use sm3::{Sm3, Digest};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        hash_stdin()?;
    }else if args.len() == 2 && args[1] == "-" {
        hash_stdin()?;
    }else if args.len() == 2 && args[1] == "--help" {
        print_help();
    } else {
        hash_files(&args[1..])?;
    }

    Ok(())
}

fn hash_stdin() -> Result<()> {
    let mut buffer = [0u8; 1024];
    let mut hasher = Sm3::new();

    loop {
        let bytes_read = io::stdin().read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    println!("{:x}  -", hash);
    Ok(())
}

fn hash_files(filenames: &[String]) -> Result<()> {
    let mut buffer = [0u8; 1024];
    let mut hasher = Sm3::new();

        for filename in filenames {
            let mut file = File::open(filename)?;
            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            let hash = hasher.clone().finalize();
            println!("{:x}  {}", hash, filename);
            hasher.reset();
        }
    Ok(())
}

fn print_help() {
    println!("Usage: sm3sum [OPTION]... [FILE]...");
    println!("Print SM3 checksums.");
    println!("");
    println!("With no FILE, or when FILE is -, read standard input.");
    println!("");
    println!("Options:");
    println!("  --help    display this help and exit");
}
