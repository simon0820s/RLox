use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let source: String = std::fs::read_to_string(path).expect("Failed to read file");
    run(&source);
}

fn run_prompt() {
    let stdin: io::Stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line: String = String::new();

        if stdin.read_line(&mut line).unwrap() == 0 {
            break;
        }

        run(&line);
    }
}

fn run(source: &str) {
    println!("{}", source);
}
