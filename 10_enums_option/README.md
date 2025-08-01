# Nullability

Our implementation of the `assigned` method is fairly blunt: panicking for to-do and done tickets is far from ideal.\
We can do better using **Rust's `Option` type**.

## `Option`

`Option` is a Rust type that represents **nullable values**.\
It is an enum, defined in Rust's standard library:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` encodes the idea that a value might be present (`Some(T)`) or absent (`None`).\
It also forces you to **explicitly handle both cases**. You'll get a compiler error if you are working with
a nullable value and you forget to handle the `None` case.\
This is a significant improvement over "implicit" nullability in other languages, where you can forget to check
for `null` and thus trigger a runtime error.

## `Option`'s definition

`Option`'s definition uses a Rust construct that you haven't seen before: **tuple-like variants**.

### Tuple-like variants

`Option` has two variants: `Some(T)` and `None`.\
`Some` is a **tuple-like variant**: it's a variant that holds **unnamed fields**.

Tuple-like variants are often used when there is a single field to store, especially when we're looking at a
"wrapper" type like `Option`.

### Tuple-like structs

They're not specific to enums—you can define tuple-like structs too:

```rust
struct Point(i32, i32);
```

You can then access the two fields of a `Point` instance using their positional index:

```rust
let point = Point(3, 4);
let x = point.0;
let y = point.1;
```

### Tuples

It's weird to say that something is tuple-like when we haven't seen tuples yet!\
Tuples are another example of a primitive Rust type.
They group together a fixed number of values with (potentially different) types:

```rust
// Two values, same type
let first: (i32, i32) = (3, 4);
// Three values, different types
let second: (i32, u32, u8) = (-42, 3, 8);
```

The syntax is simple: you list the types of the values between parentheses, separated by commas.
You can access the fields of a tuple using the dot notation and the field index:

```rust
assert_eq!(second.0, -42);
assert_eq!(second.1, 3);
assert_eq!(second.2, 8);
```

Tuples are a convenient way of grouping values together when you can't be bothered to define a dedicated struct type.

# Unwrapping

`Ticket::new` now returns a `Result` instead of panicking on invalid inputs.\
What does this mean for the caller?

## Failures can't be (implicitly) ignored

Unlike exceptions, Rust's `Result` forces you to **handle errors at the call site**.\
If you call a function that returns a `Result`, Rust won't allow you to implicitly ignore the error case.

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// This won't compile: we're not handling the error case.
// We must either use `match` or one of the combinators provided by 
// `Result` to "unwrap" the success value or handle the error.
let number = parse_int("42") + 2;
```

## You got a `Result`. Now what?

When you call a function that returns a `Result`, you have two key options:

- Panic if the operation failed.
  This is done using either the `unwrap` or `expect` methods.
  ```rust
  // Panics if `parse_int` returns an `Err`.
  let number = parse_int("42").unwrap();
  // `expect` lets you specify a custom panic message.
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- Destructure the `Result` using a `match` expression to deal with the error case explicitly.
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
