struct Point {
    x: f32,
    y: f32
}

pub fn struct_data() {
    let mut mut_point: Point = Point { x: 0.0, y: 0.0 };
    let immut_point: Point = Point { x: 1.0, y: 1.0 };

    println!("Before : {} {}", mut_point.x, mut_point.y);
    
    mut_point.y += 1.0;

    println!("After : {} {}", mut_point.x, mut_point.y);
    println!("Point : {} {}", immut_point.x, immut_point.y);

    print_pattern_matching(Point { x: 0.0, y: 0.0 });
    print_pattern_matching(Point { x: 1.0, y: 0.0 });
    print_pattern_matching(mut_point);
    print_pattern_matching(immut_point);
}

fn print_pattern_matching(point: Point) {
    match point {
        Point { x: 0.0, y: 0.0} => { println!("all zero") }
        Point { x: 0.0, y: yy } => { println!("y : {}", yy) }
        Point { x: xx, y: 0.0 } => { println!("x : {}", xx) }
        Point { x: xx, y : yy} => { println!("x : {} , y : {}", xx, yy) }
    }
}