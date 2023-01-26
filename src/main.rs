use std::fs;
use std::path::Path;
use text_colorizer::*;
use clap::Parser;
use walkdir::WalkDir;

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

    // potentially use PathBuf.set_file_name()
}

fn rename_and_copy_files(src_path: &str, dst_path: &str) {
    let source_path = Path::new(src_path);
    let dest_path = Path::new(dst_path);
    eprintln!("Destination path at beginning: {}", dst_path);
    if !source_path.exists() || !dest_path.exists() {
        eprintln!("{}: Invalid path entered", "[*]".red());
        std::process::exit(1);
    }

    let mut str_path = "";
    if dst_path.ends_with(std::path::MAIN_SEPARATOR) {
        eprintln!("ENDED WITH SLASH");
         str_path = match dst_path.strip_suffix(std::path::MAIN_SEPARATOR) {
            Some(s) => s,
            None => {
                eprintln!("{}: Failed to strip trailing slash from path", "[*]".red());
                std::process::exit(1);
            }
        };
    } else {
        str_path = dst_path;
    }

    let mut curr_album_name = "";
    let mut curr_artist_name = "";
    for entry in WalkDir::new(source_path) {
        let entry = match entry {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: Failed to read folder: {}", "[*]".red(), e);
                std::process::exit(1);
            },
        };

        if entry.file_type().is_dir() {
            continue;
        }

        let whole_path = entry.path();
        let old_song_name = entry.file_name();
        let whole_path = match whole_path.to_str() {
            Some(s) => s,
            None => {
                eprintln!("{}: Failed to convert Path to &str", "[*]".red());
                std::process::exit(1);
            },
        };

        let mut path_components: Vec<&str> = whole_path.split(std::path::MAIN_SEPARATOR).collect();
        // eprintln!("Components: {:?}", path_components);

        // pop the file name first and retrieve the album name and artist name
        path_components.pop();
        curr_album_name = match path_components.get(path_components.len() - 1) {
            Some(s) => s,
            None => {
                eprintln!("{}: Failed to retrieve album name", "[*]".red());
                std::process::exit(1);
            }
        };

        curr_artist_name = match path_components.get(path_components.len() - 2) {
            Some(s) => s,
            None => {
                eprintln!("{}: Failed to retrieve artist name", "[*]".red());
                std::process::exit(1);
            }
        };

        eprintln!("Artist name: {}, album name: {}", curr_artist_name, curr_album_name);

        // build destination path
        let dst_components = [ str_path, curr_artist_name, curr_album_name ];

        // https://www.reddit.com/r/rust/comments/bv51ul/ascii_char_to_str/
        let mut c = [0; 1];
        let final_dst_path = dst_components.join(std::path::MAIN_SEPARATOR.encode_utf8(&mut c));
        eprintln!("Final destination path: {}", final_dst_path);

        let final_dst_path = Path::new(final_dst_path.as_str());
    }
    
}