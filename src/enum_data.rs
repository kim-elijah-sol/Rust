use std::f32::consts::PI;

struct Point {
    x: f32,
    y: f32
}

enum Shape {
    Circle { radius: f32 },
    Rectangle { top_left: Point, bottom_right: Point }
}

pub fn enum_data() {
    let circle: Shape = Shape::Circle { radius: 8.0 };
    println!("circle area : {}" , get_area(circle));
    let rectangle: Shape = Shape::Rectangle { top_left: Point { x: -2.0, y: -2.0 }, bottom_right: Point { x: 2.0, y: 2.0 } };
    println!("rectangle area : {}", get_area(rectangle));
}

fn get_area(shape: Shape)-> f32 {
    match shape { 
        Shape::Circle { radius: r } => r * PI,
        Shape::Rectangle { top_left, bottom_right } => {
            (bottom_right.x - top_left.x) * (bottom_right.y - top_left.y)
        }
    }
}