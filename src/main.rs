use clap::Parser;
use clap_derive::Parser;
use mail_parser::Message;
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    /// Input file.
    #[arg(short, long)]
    input: PathBuf,

    /// Output file, if not given, `stdout` is used.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let opt = Args::parse();
    let input =
        fs::read(&opt.input).unwrap_or_else(|_| panic!("Unable to read file {:?}", opt.input));

    let message = Message::parse(&input[..]).expect("Failed to parse message");

    let json_string = serde_json::to_string_pretty(&message)
        .map_err(|e| panic!("Unable to generate JSON from email with error: {}", e))
        .unwrap();

    if let Some(output) = opt.output {
        fs::write(output, json_string)
            .map_err(|e| panic!("Unable to write to file with error: {}", e))
            .unwrap();
    } else {
        println!("{}", json_string);
    }
}
