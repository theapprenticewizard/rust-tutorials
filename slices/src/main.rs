fn main() {
    let s = String::from("Hello World!");
    let hello = &s[0..4];
    let world = &s[6..11];

    let lorem = String::from("Lorem Ipsum");
    let another = String::from("And another one!");
    let word = String::from("word");

    println!("{}, {}~!", hello, world);
    println!("first: {}", first_word(&word));
    println!("second: {}", second_word(&word));
    println!("{}", first_word(&lorem));
    println!("{}", first_word(&another));
    println!("{}", second_word(&another));
    println!("{}", second_word(&lorem));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // returns a slice that represents the whole array
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {

        if item == b' ' && start > 0  {
            return &s[start+1..i];
        } else if item == b' ' {
            start = i;
        }
    }

    ""
}