use anyhow::Result;
use clap::{Arg, ArgAction, Command };

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
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


fn run(mut args: Args) -> Result<()> {
    if [args.words, args.bytes, args.chars, args.lines]
        .iter
        ().all(|v| v== &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    println!("{args:#?}");
    Ok(())
}

fn main() {
    if let Err(err) = run(get_args()) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
