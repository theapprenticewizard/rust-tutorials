fn main() {

    let x = 3;
    let x = x * 3;
    let x = x + 2;

    // prints the shadowed variable x (11)
    println!("{}", x);

    let x = "foobar!";

    // as shadowing allows for overwriting the underlying type
    // the compiler will happily shadow this.
    println!("{}", x)
}
