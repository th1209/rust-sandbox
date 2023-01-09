use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Toshiki Hata",
    about = "Simple RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        // println!("No file is specified")
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}


// ※トレイト境界は､以下のように書いても同じ
// fn run<R: BufRead>(reader: R, verbose: bool) { /**/ }
fn run<R>(reader: R, verbose: bool)
where
    R: BufRead,
{
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
