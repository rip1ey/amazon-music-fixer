use std::fs;
use std::path::Path;
use text_colorizer::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Amazon Music Fixer")]
#[command(about = "Removes the extraneous information from Amazon Music downloads")]
struct Args {
    /// The path to the AmazonMusic folder
    #[arg(short, long)]
    source: String,

    /// The path to write the updated music to
    #[arg(short, long)]
    dest: Option<String>,

    /// Option for modifying song titles in place (default: false)
    #[arg(long, default_value_t = false)]
    modify_in_place: bool,
}

fn main() {
    let args = Args::parse();
    println!("Arguments passed in: {:?}", args);

    let source = args.source;
    let dest = args.dest;
    let in_place = args.modify_in_place;

    // the destination option and modify-in-place option are mutually exclusive
    if in_place {
        match dest {
            Some(_) => {
                eprintln!("{}: Cannot pass both a destination path and modify in place flag", "[*]".red());
                std::process::exit(1);
            },
            None => {
                eprintln!("{}: Renaming files in place at {}", "[+]".green(), source);
                rename_in_place(&source);
                std::process::exit(0);
            },
        }
    }

    if let Some(dst) = dest {
        rename_and_copy_files(&source, &dst);
    } else {
        eprintln!("{}: Invalid destination path entered", "[*]".red());
        std::process::exit(1);
    }
}

fn rename_in_place(path: &str) {
    let source_path = Path::new(path);

    if !source_path.exists() {
        eprintln!("{}: Source path does not exist. Please enter a valid path to check", "[*]".red());
        std::process::exit(1);
    }
}

fn rename_and_copy_files(src_path: &str, dst_path: &str) {
    let source_path = Path::new(src_path);
    let dest_path = Path::new(dst_path);

    
}