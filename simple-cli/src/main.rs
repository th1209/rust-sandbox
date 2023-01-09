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

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }
    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula
            .split_whitespace()
            // popでstackの末尾から操作していくので､逆順にする
            .rev()
            // colletは､イテレータをコレクションに変換する
            // _は､Rustコンパイラ側で適切な方に推論してくれる
            .collect::<Vec<_>>();
        self.eval_impl(&mut tokens)
    }

    fn eval_impl(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token:{}", token),
                };
                stack.push(res);
            }

            // verbose表示
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax");
        }
    }
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
    let calcurator = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calcurator.eval(&line);
        println!("{}", answer);
    }
}
