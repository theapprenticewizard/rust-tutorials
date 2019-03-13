fn main() {
    
    let x = 5;

    if x >= 3 {
        println!("number was greater than or equal to 3!");
    } else {
        println!("numnber was less than 3");
    }

    // parentheses work, but not recommended
    // also else if an else are always a thing
    if 1 == 3 {
        println!("I am a pointless statement!");
    } else if 2 == 3 {
        println!("huh!?");
    } else if 3 == 3 {
        println!("that should work!");
    } else {
        println!("no one should see this, dududududu");
    }

    // now for something fancy!

    let k = if 1==2 {
        3
    } else {
        5
    };

    println!("my magic number was... {}", k);

    // while(true) standin
    // loop {
    //     println!("spam");
    // }

    let mut countdown = 3;

    // standard while loop, just no paren
    while countdown > 0 {
        println!("{}", countdown);
        countdown -= 1; // no i-- or i++ or ++i or --i semantics!
    }

    println!("liftoff!");

    let pokemon = ["pichu", "pikachu", "raichu", "ditto"];
    let mut index = 0;

    while index < pokemon.len() {
        println!("{}", pokemon[index]);
        index += 1;
    }

    println!("\nNow for a cleaner way!\n");

    // cleaner way! for-in loop
    for pok in pokemon.iter() {
        println!("{}", pok);
    }


    // better countdown

    for n in (0..6).rev() {
        println!("{}", n);
    }

    println!("liftoff!");
}
