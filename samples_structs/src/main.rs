fn main() {
    // NOTE: rust doesn't support method overloading
    // typically you would instead use tuples to represent
    // overloaded parameters instead.

    let width = 30;
    let height = 50;
    let rect = (30, 50);
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    let _box = Box(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_worst(width, height)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_better(rect)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_best(&rectangle)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_perhaps_better(&_box)
    );

    // better printing using this technique!
    println!("debug: {:?}", rectangle);
}

// first version: doesn't correlate height and width
fn area_worst(width: u32, height: u32) -> u32 {
    width * height
}

// second version correlates it, but not semantic
fn area_better(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_best(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

// box is a keyword, so we can't use that
fn area_perhaps_better(_box: &Box) -> u32 {
    _box.0 * _box.1
}

// tuple struct example
struct Box(u32, u32);

// deriving the debug trait, using this 
// annotation/attribute like thing!
// Debug is likely the name of another Struct... 
// `struct Debug { }` debug should be empty because it
// is a Unit Struct likely
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}