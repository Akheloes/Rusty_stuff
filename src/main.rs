#[derive(Debug)]
enum Command {
    Query,
    Mutate,
    BiSequence { first: String, second: String },
}

fn main() {
    use crate::Command::*;

    let called = Query;

    match called {
        Query => println!("Called Query"),
        Mutate => println!("Called Mutate"),
        BiSequence { first, second } => println!("Called BiSequence {{ first: {}, second: {}}}", first, second),
    }
}