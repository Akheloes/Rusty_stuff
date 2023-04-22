#![allow(dead_code)]

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
}

fn main() {
    let number_zero = Number::Zero as i8;
    let number_one = Number::One as i8;
    let red_color: i32 = Color::Red as i32;

    println!("{}", number_zero);
    println!("{}", number_one);
    println!("{}", red_color);
}