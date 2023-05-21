fn main() {
    let mut s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of {s} is {len}");
    change(&mut s);
    println!("The length of {s} is {}", calculate_length(&s));

    // let r1 = &mut s;
    // s.push_str("!!");
    // println!("{}", *r1);
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{hello} {world}");
    let first_word = get_first_word(&s);
    println!("{first_word}");
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..] // convert the whole string to a slice
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn take_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn take_and_give_back(some_string: String) -> String {
//     println!("{some_string}");
//     some_string
// }

// fn make_copy(some_integer: i32) {
//     println!("{some_integer}");
// }
