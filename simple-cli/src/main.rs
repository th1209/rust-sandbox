use anyhow::{bail, ensure, Context, Result};
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
    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula
            .split_whitespace()
            // popでstackの末尾から操作していくので､逆順にする
            .rev()
            // colletは､イテレータをコレクションに変換する
            // _は､Rustコンパイラ側で適切な方に推論してくれる
            .collect::<Vec<_>>();
        self.eval_impl(&mut tokens)
    }

    fn eval_impl(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            // verbose表示
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        // println!("No file is specified")
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

// ※トレイト境界は､以下のように書いても同じ
// fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> { /**/ }
fn run<R>(reader: R, verbose: bool) -> Result<()>
where
    R: BufRead,
{
    let calcurator = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line?;
        match calcurator.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(())
}

// cfgアトリビュートはコンディショナル的な属性. ここではcargo testの時のみ有効になる
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calclulator = RpnCalculator::new(false);
        assert_eq!(calclulator.eval("5").unwrap(), 5);
        assert_eq!(calclulator.eval("50").unwrap(), 50);
        assert_eq!(calclulator.eval("-50").unwrap(), -50);

        assert_eq!(calclulator.eval("2 3 +").unwrap(), 5);
        assert_eq!(calclulator.eval("2 3 -").unwrap(), -1);
        assert_eq!(calclulator.eval("2 3 *").unwrap(), 6);
        assert_eq!(calclulator.eval("2 3 /").unwrap(), 0);
        assert_eq!(calclulator.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    fn test_ng() {
        let calclulator = RpnCalculator::new(false);
        assert!(calclulator.eval("").is_err());
        assert!(calclulator.eval("1 1 1 +").is_err());
        assert!(calclulator.eval("+ 1 1").is_err());
        
    }
}
