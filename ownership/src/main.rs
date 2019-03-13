fn main() {
    let s = String::from("Hello!");
    let mut other_string = String::from("World!");

    // the pointer for s is gone forever now (until app ends)
    take_ownership(s);

    other_string = borrow_ownsership(other_string);
    println!("ownership returned: {}", other_string);

    let i = 24;

    make_copy(i);

    let my_string = String::from("Hello!");
    let tup = calculate_length(my_string);

    println!("{} has {} characters!", tup.0, tup.1);
}


/// takes a String's pointer's ownership, prints the value
/// then goes out of scope. (dropping the ownership)
fn take_ownership(s: String) {
    println!("{}", s);
}


fn borrow_ownsership(s: String) -> String {
    println!("I have borrowed {}", s);
    s
}

/// Types with known fixed sizes use the stack, and therefor
/// do not care about ownership.
fn make_copy(i: i32) {
    println!("{}", i);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
