use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct EvenNumber<i32> {
    value: i32
}

impl TryFrom<i32> for EvenNumber<i32> {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (value % 2 == 0) {
            
            return Ok(EvenNumber { value });
        } else {
            
            return Err(());
        }
    }

}

fn main() {

    let even_number = EvenNumber::try_from(8i32);
    println!("Successful from even {:?}", even_number);
    
    let extract_even_number: Result<EvenNumber<i32>, ()> = 8i32.try_into();
    println!("Successful into even {:?}", extract_even_number);
    
    let odd_number = EvenNumber::try_from(7i32);
    println!("failed from odd {:?}", odd_number);
    
    let extract_odd_number: Result<EvenNumber<i32>, ()> = 7i32.try_into();
    println!("failed into odd {:?}", extract_odd_number);
}