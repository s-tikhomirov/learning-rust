fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    };
    let result = counter;
    println!("{result}");
}
