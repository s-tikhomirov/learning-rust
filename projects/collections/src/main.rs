use std::hash::Hash;


fn vectors() -> () {
    let v_empty: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

    // let third: &i32 = &v[2];
    // more safe:
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
}

fn strings() -> () {
    let s1 = String::from("Hello, ");
    let s2 = "world!".to_string();  // the same as ::from
    let s3 = s1 + &s2;
    println!("{s3}");
    println!("{s2}");  // s2 wat provided by reference and is still alive
    //println!("{s1}");  // this fails: s1 has been consumed!
}

fn strings_clone() -> () {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s11 = s1.clone();
    let s4 = s11 + &s2;
    println!("{s1}"); 
}

fn strings_format() -> () {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    println!("{s1} {s2} {s3}"); // nothing was consumed!
}

fn hashmaps() -> () {
    use std::collections::HashMap;
    // we don't specifry types here...
    let mut scores = HashMap::new();
    // because they are inferred from here:
    scores.insert(String::from("Blue"), 10);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    scores.insert(String::from("Blue"), 25);
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", scores);
    // insert if key didn't exist yet
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);  // won't update the value
    println!("{:?}", scores);
}

fn hashmaps_counter() -> () {
    use std::collections::HashMap;
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    vectors();
    strings();
    strings_clone();
    strings_format();
    hashmaps();
    hashmaps_counter();
}
