pub fn tuple_data () {
    let mut t: (i32, i32, f64) = (0, 0, 0.0);
    print_pattern_matching(t);

    t.0 = 10;
    print_pattern_matching(t);

    t.1 = 20;
    print_pattern_matching(t);

    t.2 = 30.0;
    print_pattern_matching(t);
}

fn print_pattern_matching(t: (i32, i32, f64)) {
    match t {
        (0, _, _) => println!("first element is zero"),
        (_, 0, _) => println!("second element is zero"),
        (_, _, 0.0) => println!("third element is zero"),
        (x, y, z) => println!("{}", x + y + (z as i32))
    }
}