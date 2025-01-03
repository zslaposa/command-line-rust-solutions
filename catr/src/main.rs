use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
    .version("0.1.0")
    .about("Rust version of cat")
    .arg(
        Arg::new("files")
        .value_name("FILE")
        .help("Input file(s)")
        .num_args(1..)
        .default_value("-")
    )
    .arg(
        Arg::new("number")
            .long("number")
            .short('n')
            .help("Number lines")
            .action(ArgAction::SetTrue)
            .conflicts_with("number_nonblank"),

    )
    .arg(
        Arg::new("number_nonblank")
            .long("number-nonblank")
            .short('b')
            .help("Number non-blank lines")
            .action(ArgAction::SetTrue)            

    )
    .get_matches();

    Args { 
        files: matches.get_many("files").unwrap().cloned().collect(), 
        number_lines: matches.get_flag("number"), 
        number_nonblank_lines: matches.get_flag("number_nonblank")
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;

                    if args.number_lines {
                        println!("{:>6}\t{line}", line_num + 1)
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!()
                        } else {
                            prev_num += 1;
                            println!("{prev_num:>6}\t{line}")
                        }
                    } else {
                        println!("{line}")
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(get_args()) {        
        std::process::exit(1);
    }
}
