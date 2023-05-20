use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}");
    
    // let mut a_binding;
    // a_binding = String::from("qwe");
    // println!("{a_binding}");


    let mut array_len = String::new();
    io::stdin()
        .read_line(&mut array_len)
        .expect("Failed to read line");
    
    let array_len = array_len
        .trim()
        .parse()
        .expect("Index must be an integer");

    let a = [3, array_len];

    println!("{:?}", a);

    let element = a[5];   // this will panic due to out-of-bound access??
    println!("The 5th element is {}", element);
    


}
