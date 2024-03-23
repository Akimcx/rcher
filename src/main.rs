use std::{
    hash::{DefaultHasher, Hasher},
    io,
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Cli {
    // #[arg(short,long)]
    path: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;

    let mut hasher = DefaultHasher::new();
    if path.len() == 0 {
        println!("Enter `quit` to quit the program or hit Ctrl+c");
        loop {
            let mut input = String::new();
            loop {
                match io::stdin().read_line(&mut input) {
                    Ok(_) => break,
                    Err(err) => println!("ERROR: {err}"),
                }
            }
            let input = input.trim();
            if input == "quit" {
                break;
            }
            hasher.write(input.as_bytes());
            println!("{}", hasher.finish());
            hasher = DefaultHasher::new();
        }
        return;
    }
    for ele in path {
        let pathbuf = PathBuf::from(&ele);
        if pathbuf.exists() {
            // TODO: improve error handeling
            match std::fs::read(&pathbuf) {
                Ok(content) => {
                    hasher.write(&content);
                    println!("{}", hasher.finish());
                }
                Err(error) => {
                    eprintln!("{:?} {}", pathbuf, error);
                }
            }
        } else {
            hasher.write(ele.as_bytes());
            println!("{}", hasher.finish());
        }
        hasher = DefaultHasher::new();
    }
}
