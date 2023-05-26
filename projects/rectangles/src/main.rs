use rectangles::Rectangle as Rectangle;

fn main() {
    let scale = 2;
    let rect1 = Rectangle::new(
        30 * scale,
        50,
    );
    let rect1 = dbg!(rect1);
    println!("The area of {:#?} is {}", rect1, rect1.area());

    let rect2 = Rectangle::new(
        40,
        30,
    );
    let square1 = Rectangle::square(100);
    println!("{:?} can hold {:?}? - {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("{:?} can hold {:?}? - {}", square1, rect2, square1.can_hold(&rect2));
}
