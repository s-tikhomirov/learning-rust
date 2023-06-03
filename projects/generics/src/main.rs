fn main() {
    let list = vec![1,4,2,6,2,2];
    println!("{}", largest(&list));
    let list2 = vec![10,11,12,15,11,19];
    println!("{}", largest(&list2));
    let s = Point{x: 1, y: 2};
    println!("{} {}", s.x, s.y);
}

struct Point<T, U> {
	x: T,
	y: U,
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}