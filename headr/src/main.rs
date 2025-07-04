use anyhow::Result;
use clap::{Arg, Command };
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

fn get_args() -> Args {
    let arguments = Command::new("headr")
        .version("0.1.0")
        .about("Rust version of head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .help("Number of lines")
                .num_args(1)
                .value_parser(clap::value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .value_parser(clap::value_parser!(u64).range(1..))
                .help("Number of bytes"),
        )
        .get_matches();

    Args {
        files: arguments.get_many("files").unwrap().cloned().collect(),
        lines: arguments.get_one("lines").cloned().unwrap(),
        bytes: arguments.get_one("bytes").cloned(),
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for file_name in args.files {
        match open(&file_name) {
            Err(err) => eprintln!("{file_name}: {err}"),
            Ok(mut file) => {
                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buffer)?;
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    )

                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run(get_args()) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
