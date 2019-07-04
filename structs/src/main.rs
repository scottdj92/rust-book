struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let user = User {
        email: String::from("someemail@mail.com"),
        username: String::from("someusername"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        email: String::from("otheremail@mail.com"),
        username: String::from("username"),
        ..user,
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}
