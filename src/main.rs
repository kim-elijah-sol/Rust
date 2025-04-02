mod let_const;
mod data_type;
mod pattern_matching;
mod struct_data;

fn main() {
    struct_data::struct_data();
    println!();
    pattern_matching::pattern_matching();
    println!("{}", pattern_matching::angle((0.0,-1.5)));
    println!("{}", pattern_matching::angle((0.0,2.0)));
    println!("{}", pattern_matching::angle((1.0,2.0)));
    println!();
    data_type::data_type();
    println!();
    let_const::let_const();
}