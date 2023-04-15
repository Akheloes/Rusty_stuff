use std::f32;

fn main() {
    let top_point: Point = Point { x: 2.0, y: 3.0 };
    let bottom_point: Point = Point { x: 1.0, y: 2.0 };

    println!("{}", rectangle_area(top_point, bottom_point));
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn rectangle_area(top: Point, bottom: Point) -> f32 {
    let Point { x: top_x, y: top_y } = top;
    let Point { x: bottom_x, y: bottom_y } = bottom;

    return axial_distance(top_x, bottom_x) * axial_distance(top_y, bottom_y);
}

fn axial_distance(top_x: f32, bottom_x: f32) -> f32 {
    f32::abs(top_x - bottom_x)
}