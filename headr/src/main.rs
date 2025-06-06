use clap::{Arg, Command };

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

fn main() {
    let args = get_args();
    println!("{:#?}", args);
}
