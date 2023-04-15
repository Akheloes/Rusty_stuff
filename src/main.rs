#[derive(Debug)]
enum Command {
    Query,
    Mutate,
    BiSequence { first: String, second: String },
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
    handle(Command::BiSequence { first: String::from("First command"), second: String::from("Second command") });
}