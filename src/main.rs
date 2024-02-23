use std::{hash::{DefaultHasher, Hasher}, io, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Cli {
    // #[arg(short,long)]
    path: Vec<String>
}

fn main() {
    let args = Cli::parse();
    let path = args.path;

    println!("Number of arguments: {}", path.len());
    let mut hasher = DefaultHasher::new();
    if path.len() == 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Hello");
        hasher.write(input.as_bytes()); 
        println!("{}",hasher.finish());
        return;
    }
    for ele in path {
        let pathbuf = PathBuf::from(&ele);
        if pathbuf.exists() {
            let content = std::fs::read(pathbuf).expect("Cannot read file");
            println!("content: {:?}", content);
            hasher.write(&content);
        }else {
            hasher.write(ele.as_bytes());
        }
        println!("hashee: {}", hasher.finish());
    }
}

