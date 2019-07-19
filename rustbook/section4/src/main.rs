fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = word_separator(&my_string[..]);
    println!("{}",word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = word_separator(&my_string_literal[..]);
    println!("{}",word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = word_separator(my_string_literal);
    println!("{}",word);

    let word = word_separator(&my_string_literal[word.len()+1..]);
    println!("{}",word);
}

fn word_separator(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}
