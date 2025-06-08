
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Point<T> is a generic struct that can hold any type T
    let p1 = Point { x: 5, y: 10 };
    println!("p1.x = {}", p1.x());
    //let wont_work = Point { x: 5, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("p2.distance_from_origin = {}", p2.distance_from_origin());

    // Point2<T, U> is a generic struct that can hold two different types T and U
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // Point3<X1, Y1> is a generic struct that can hold two different types X1 and Y1
    let p31 = Point3 { x: 5, y: 10.4 };
    let p32 = Point3 { x: "Hello", y: 'c' };
    let p33 = p31.mixup(p32);
    println!("p33.x = {}, p33.y = {}", p33.x, p33.y);

    // Monomorphization
    let integer = Some(5);
    let float = Some(5.0);

    // Monomorphization works like this:
    /*
        enum Option_i32 {
            Some(i32),
            None,
        }

        enum Option_f64 {
            Some(f64),
            None,
        }

        fn main() {
            let integer = Option_i32::Some(5);
            let float = Option_f64::S
    */

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// To compile this we need to restrict the types valid for T to only those 
// that implement PartialOrd: the standard library implements on both i32 and char.
//fn largest<T>(list: &[T]) -> &T {
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}