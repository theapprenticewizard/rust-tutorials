fn main() {
    println!("references!");
    let mut s = String::from("Hello, World!");

    /// caller passes reference of &s, rather than just s
    println!("{} has {} characters", s, calculate_length(&s));

    do_evil(&mut s);
    println!("{}", s);
}

// the following function if called will leave a dangled pointer.
// fn dangle() -> &String {
//     let s = String::from("Hello!");

//     &s
// }

/// function requiring references, marks as so with &Type
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn do_evil(s: &mut String) {
    s.push_str(", evil!");
    println!("{}", s);
}
