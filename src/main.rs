use clap::Parser;
use std::process::Command;
use std::str;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    let output = run_app().unwrap().stdout;

    let out_str = str::from_utf8(&output).unwrap();

    println!("{}", out_str)
}

fn run_app() -> Result<std::process::Output, std::io::Error> {
    return Command::new("git").arg("remote").arg("-v").output();
}
