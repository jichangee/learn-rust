struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        email: String::from("moxuy@gmail.com"),
        username: "moxuy",
        active: true,
        sign_in_count: 1
    };
    let mut s = String::from("hello world!");
    let word = first_word(&s);
    s.clear();
    println!("word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}