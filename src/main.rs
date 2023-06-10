use std::fmt::Display;


struct Circle<f32> {
    radius: f32
}

impl Display for Circle<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        return write!(f, "Cercle de rayon {:?}", self.radius);
    }
}

fn main() {
    let circle = Circle { radius : 7.77 };

    println!("{:?}", circle.to_string());
}