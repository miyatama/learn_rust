use clap::{Parser};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

struct RpnCalclator(bool);

impl RpnCalclator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
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
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntacx")
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "calculate app",
    version = "0.0.1",
    author = "n.miyata080825@gmail.com",
    about = "calculate command line app"
)]
struct Opts {
    #[arg(short, long)]
    verbose: bool,

    #[arg(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match open(&opts.formula_file) {
        Err(err) => eprintln!("failed to open {}: {}", opts.formula_file.unwrap(), err),
        Ok(reader) => run(reader, opts.verbose).unwrap(),
    }
}

fn run(reader: Box<dyn BufRead>, verbose: bool) -> MyResult<()> {
    let calc = RpnCalclator::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() <= 0 {
            break;
        }
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
    Ok(())
}

fn open(filename: &Option<String>) -> MyResult<Box<dyn BufRead>> {
    match filename {
        Some(filename) => Ok(Box::new(BufReader::new(File::open(filename)?))),
        _ => Ok(Box::new(BufReader::new(io::stdin()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalclator(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-50"), -50);
        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
    }
}