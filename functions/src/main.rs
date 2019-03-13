
///
///  Heyo! This is a rust doc!
///
fn main() {
    println!("Hello, world!");
    another_function();
    println!("{}", add(3, 4));
    print_one_plus(3);
    println!("{}", double(8));
}
fn another_function() {
    println!("By convention Rust's functions are snake case!");
}


// standard format for a function, note: -> i32 is the return type
// by default the return type of a function is void!
fn add(x: i32, y:i32) -> i32 {
    return x + y;
}

// void return type
fn print_one_plus(x: i32) {
    println!("{} + 1 = {}", x,  x + 1);
}

// returns implicitly the last statement!
// does not work if you use a semicolon!
fn double(x: i32) -> i32 {
    x * 2
}

