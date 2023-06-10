// Code some rust and execute 'cargo run'
#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(v: i32) -> Self {
        return Number { value: v };
    }
}

fn main() {
    let number = Number::from(7i32);
    let another_number: Number = 17i32.into();

    println!("Number 7 comes from {:?}", number);
    println!("Number 7 comes from {:?}", another_number);
}