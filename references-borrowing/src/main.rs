fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("the length of '{}' is {}", s1, len);

    let word = first_word("hello world");

    println!("first word: {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     if (item == b' ') {
    //         return i;
    //     }
    // }

    // s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
