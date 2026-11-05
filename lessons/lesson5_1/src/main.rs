//Default Structure
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

//Unnamed Structure
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Always Equal Structure
struct AlwaysEqual;

fn main() {
    println!("Hello, world!");

    //Create structure object
    let mut user1 = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1
    };
    println!(
        "User Structure: {}, {}, {}, {}", 
        user1.active, 
        user1.username, 
        user1.email, 
        user1.sign_in_count
    );

    //Change date
    user1.email = String::from("anotheruser@example.com");
    user1 = build_user(
        String::from("someuser"),
        String::from("exampleemail@example.com")
    );

    //Hard recreate
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("omg@example.com"),
        sign_in_count: user1.sign_in_count + 1
    };

    //Easier recreate
    let user3 = User {
        email: String::from("omg@exmaple.com"),
        ..user2
    };
    println!(
        "User Structure: {}, {}, {}, {}", 
        user3.active, 
        user3.username, 
        user3.email, 
        user3.sign_in_count
    );

    //Create unnamed structure
    let _black = Color(0, 0, 0);
    let _null_point = Point(0, 0, 0);

    //Create Always Equal Structure
    let _subject = AlwaysEqual;

}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}