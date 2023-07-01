use std::f64;
use std::i16;

struct Point {
    x: i16,
    y: i16, 
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = other.x;
        let y2 = other.y;

        return (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
    }

    fn new(x: i16, y: i16) -> Point {
        return Point { x, y };
    }
}

struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    fn new(a: Point, b: Point) -> Segment {
        return Segment { a, b }
    }

    fn length(&self) -> f64 {
        return self.a.distance(&self.b);
    }
}


fn main() {
    let p1 = Point::new(1, 1);
    let p2 = Point { x: 0, y: 0 };
    let distance = p1.distance(&p2);

    let segment: Segment = Segment::new(p1, p2);
    let segment_length = segment.length();

    println!("Distance: {}", distance);
    println!("Segment length: {}", segment_length);
}