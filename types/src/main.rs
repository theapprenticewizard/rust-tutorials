fn main() {

    // here, the compiler needs to be told the type explicitly
    let value: i32 = "42".parse().expect("Should be a number!");
    println!("the answer to life is {}", value);

    // running the below code causes a panic, with the the message in the expect clause!
    // let other: i32 = "foo".parse().expect("Should be a number as well!");

    // initializing a tuple
    let tup: (i32, char, f64) = (500, 'A', 3.14);

    // destructuring a tuple
    let (x, y, z) = tup;

    // printing each element of a tuple
    println!("{}, {}, {}", x, y, z);

    // directly using each element of a tuple 
    println!("{}, {}", tup.0, tup.2);


    // array values are always allocated on the 'stack' rather than the heap!
    let a = [1, 2, 3, 4, 5];

    // compound types (including tuples) can't be printed directly
    // println!("{}", a);
    println!("{}", a[0]);

}
