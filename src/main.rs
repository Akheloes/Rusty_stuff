#[derive(Debug)]
enum Command {
    Query,
    Mutate,
    BiSequence { first: String, second: String },
}

impl Command {
    fn execute(&self, code: i8) {
        match self {
            Self::Query => println!("Query {}", code),
            Self::Mutate => println!("Mutation {}", code),
            Self::BiSequence { first, second } => println!("BiSequence {}", code),
        }
    }
}

fn handle(command: Command) {
    match command {
        Command::Query => println!("Execute query"),
        Command::Mutate => println!("Execute mutation"),
        Command::BiSequence { first, second } => println!("{} -> {}", first, second),
    }
}
fn main() {
    handle(Command::Query);
    handle(Command::Mutate);
    let bi_seq_command = Command::BiSequence { first: String::from("First command"), second: String::from("Second command") };
    handle(bi_seq_command);

    Command::execute(&Command::Query, 10);
}