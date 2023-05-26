
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }

    pub fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8,7);
        let smaller = Rectangle::new(5, 1);
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        assert!(!smaller.can_hold(&larger));
    }

    fn adds_two(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(adds_two(2), 4);
    }
}