fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect1 = dbg!(rect1);
    println!("The area of {:#?} is {}", rect1, rect1.area());
    let rect2 = Rectangle {
        width: 40,
        height: 30,
    };
    let square1 = Rectangle::square(100);
    println!("{:?} can hold {:?}? - {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("{:?} can hold {:?}? - {}", square1, rect2, square1.can_hold(&rect2));
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}