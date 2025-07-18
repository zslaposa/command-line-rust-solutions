use anyhow::Result;
use clap::{Arg, ArgAction, Command };
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn get_args() -> Args {
    let arguments = Command::new("wcr")
        .version("0.1.0")
        .about("Rust version of wc")
         .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(0..)
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .action(ArgAction::SetTrue)
                .help("Show line count"),
        )                
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .action(ArgAction::SetTrue)
                .help("Show word count"),
        )                
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .action(ArgAction::SetTrue)
                .help("Show byte count"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .action(ArgAction::SetTrue)
                .conflicts_with("bytes")
                .help("Show character count"),
        )        
        .get_matches();

    Args {
        files: arguments.get_many("files").unwrap().cloned().collect(),
        lines: arguments.get_flag("lines"),
        words: arguments.get_flag("words"),
        bytes: arguments.get_flag("bytes"),
        chars: arguments.get_flag("chars"),
    }
}

fn format_count(value: usize, show: bool) -> String {
    if show {
        format!("{value:>8}")
    } else {
        "".to_string()
    }
}

fn format_file_name(filename: &str) -> String {
    if filename == "-" {
        "".to_string()
    } else {
        format!(" {filename}")
    }
    
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;

        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(mut args: Args) -> Result<()> {
    if [args.words, args.bytes, args.chars, args.lines]
        .iter
        ().all(|v| v== &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    
    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                let file_info = count(file)?;
                println!("{}{}{}{}{}", 
                    format_count(file_info.num_lines, args.lines), 
                    format_count(file_info.num_words, args.words),
                    format_count(file_info.num_bytes, args.bytes),
                    format_count(file_info.num_chars, args.chars),
                    format_file_name(filename),
                );

                total_lines += file_info.num_lines;
                total_words += file_info.num_words;
                total_bytes += file_info.num_bytes;
                total_chars += file_info.num_chars;
            }
        }
    }

    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total", 
            format_count(total_lines, args.lines), 
            format_count(total_words, args.words),
            format_count(total_bytes, args.bytes),
            format_count(total_chars, args.chars),
        );
    }

    Ok(())
}

fn main() {
    if let Err(err) = run(get_args()) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::{count, FileInfo, format_count, format_file_name}; 
    use std::io::Cursor;

    
    #[test]
    fn empty_file() {
        let text = "";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 0,
            num_words: 0,
            num_chars: 0,
            num_bytes: 0,
        };
        assert_eq!(info.unwrap(), expected);
    }
    
    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn should_format_count() {
        assert_eq!(format_count(1, false), "");
        assert_eq!(format_count(3, true), "       3");
        assert_eq!(format_count(10, true), "      10");
    }

    #[test]
    fn should_format_filename() {
        assert_eq!(format_file_name("-"), "");
        assert_eq!(format_file_name("test_file.txt"), " test_file.txt");
    }

}