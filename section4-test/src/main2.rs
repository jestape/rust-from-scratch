fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

// References are mutable to adding mut. Just one mutable reference to a particular piece of data in a particular scope.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
