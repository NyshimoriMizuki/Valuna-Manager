use std::io::{self, Write};

pub struct Repl {
    version: String,
}

impl Repl {
    pub fn new(version: &str) -> Repl {
        Repl {
            version: version.to_string(),
        }
    }

    pub fn run(&self) {
        println!("Valuna Manager V{}a (by Nyshimori Mizuki)", self.version);
        println!("Type \"quit\" or \"Q\" to exit.");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).unwrap();
            match self.evaluate(&cmd) {
                command => {
                    if command == String::from("quit") || command == String::from("Q") {
                        break;
                    }
                    println!("{}", command)
                }
            }
        }
    }

    fn evaluate(&self, command: &str) -> String {
        String::from(command.trim())
    }
}
