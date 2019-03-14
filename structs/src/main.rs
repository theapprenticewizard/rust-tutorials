fn main() {

    // initializing a user...
    let my_user = User {
        email: String::from("rayd3900@gmail.com"),
        username: String::from("theapprenticewizard"),
        active: true,
        sign_in_count: 1
    };

    let mut mutable_user = User {
        email: String::from("rayd3900@gmail.com"),
        username: String::from("theapprenticewizard"),
        active: true,
        sign_in_count: 1
    };

    mutable_user.email = String::from("raymond@bluefinenterprises.com");

    let mut other_user = create_user(
        String::from("test@test.com"),
        String::from("my_username")
    );

    other_user.sign_in_count += 1;

    let init_user = create_user_using_init(
        String::from("email@email.com"), 
        String::from("my_username")
    );

    println!("{}", my_user.username);
    println!("{}", mutable_user.email);
    println!("{}", other_user.sign_in_count);
    println!("{}", init_user.username);


    // ------ Struct Update Syntax


    println!("\nStruct update Syntax\n");

    // awesome struct update syntax.
    let updated_user = User {
        email: String::from("newemail@emai..com"),
        username: String::from("new_username"),
        ..init_user        
    };

    println!("pre update mutable user: {}", mutable_user.email);

    // it's possible to update using the old object too!
    let mutable_user = User {
        email: String::from("mutable@email.com"),
        ..mutable_user
    };

    println!("updated user: {}", updated_user.username);
    println!("mutable user udpated: {}", mutable_user.email);

    // Struct Tuples

    // r, g, b
    struct Color(i32, i32, i32);
    struct Vector(i32, i32);
    let point = Vector(0, 0);
    let color = Color(255, 0, 0);

    println!("the 'r' value of color is: {}", color.0);

    // nothing we can do with this... but compiles!
    let m = Marker{};
}

struct Marker { /* used for traits... more on this later */}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}

fn create_user_using_init(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}