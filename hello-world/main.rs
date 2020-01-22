fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user1 = User {
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
}
