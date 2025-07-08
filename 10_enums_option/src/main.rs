/*
enum Option<T> {
    None,
    Some(T),
}

Included in prelude (no need for namespace).
None and Some can be used without Option::
*/

fn main() {
    let some_number = Some(5);
    //let some_other_number = Some(5);
    // let some = some_number + some_other_number; // error[E0369]: cannot add `Option<{integer}>` to `Option<{integer}>`

    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y; // error[E0277]: cannot add `Option<i8>` to `i8`

    let z = match y {
        None => None,
        Some(i) => Some(x + i), // aka = x + y
    };
}
