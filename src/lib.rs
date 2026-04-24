use std::io::{self, Write};

pub struct RLox {}

impl RLox {
    pub fn new() -> Self {
        RLox {}
    }
    pub fn run_file(&mut self, path: &str) {
        let source: String = std::fs::read_to_string(path).expect("Failed to read file");
        self.run(&source);
    }

    pub fn run_prompt(&mut self) {
        let stdin: io::Stdin = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line: String = String::new();

            if stdin.read_line(&mut line).unwrap() == 0 {
                break;
            }

            self.run(&line);
        }
    }

    fn run(&mut self, source: &str) {
        println!("{}", source);
    }
}
