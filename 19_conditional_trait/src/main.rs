use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    Pair::new(1, 2).cmp_display();
    Pair::new(1.0, 2.0).cmp_display();
    Pair::new("hello", "world").cmp_display();
    // Pair::new(1, "hello").cmp_display(); // This will not compile because i32 and &str do not implement Display and PartialOrd together.
    // Pair::new(1, 2.0).cmp_display(); // This will not compile because i32 and f64 do not implement Display and PartialOrd together.

    let pair1 = Pair::new(1, 2);
    let pair2 = Pair::new(2, 4);

    if pair1 > pair2 {
        println!("pair1 is greater than pair2");
    } else {
        println!("pair1 is not greater than pair2");
    }
}
