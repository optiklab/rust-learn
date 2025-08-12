
# Fallibility

Let's revisit the `Ticket::new` function from the previous exercise:

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: Status
    ) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}
```

As soon as one of the checks fails, the function panics.
This is not ideal, as it doesn't give the caller a chance to **handle the error**.

It's time to introduce the `Result` type, Rust's primary mechanism for error handling.

## The `Result` type

The `Result` type is an enum defined in the standard library:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

It has two variants:

- `Ok(T)`: represents a successful operation. It holds `T`, the output of the operation.
- `Err(E)`: represents a failed operation. It holds `E`, the error that occurred.

Both `Ok` and `Err` are generic, allowing you to specify your own types for the success and error cases.

## No exceptions

Recoverable errors in Rust are **represented as values**.\
They're just an instance of a type, being passed around and manipulated like any other value.
This is a significant difference from other languages, such as Python or C#, where **exceptions** are used to signal errors.

Exceptions create a separate control flow path that can be hard to reason about.\
You don't know, just by looking at a function's signature, if it can throw an exception or not.
You don't know, just by looking at a function's signature, **which** exception types it can throw.\
You must either read the function's documentation or look at its implementation to find out.

Exception handling logic has very poor locality: the code that throws the exception is far removed from the code
that catches it, and there's no direct link between the two.

## Fallibility is encoded in the type system

Rust, with `Result`, forces you to **encode fallibility in the function's signature**.\
If a function can fail (and you want the caller to have a shot at handling the error), it must return a `Result`.

```rust
// Just by looking at the signature, you know that this function 
// can fail. You can also inspect `ParseIntError` to see what 
// kind of failures to expect.
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}
```

That's the big advantage of `Result`: it makes fallibility explicit.

Keep in mind, though, that panics exist. They aren't tracked by the type system, just like exceptions in other languages.
But they're meant for **unrecoverable errors** and should be used sparingly.

# Error enums

Your solution to the previous exercise may have felt awkward: matching on strings is not ideal!\
A colleague might rework the error messages returned by `Ticket::new` (e.g. to improve readability) and,
all of a sudden, your calling code would break.

You already know the machinery required to fix this: enums!

## Reacting to errors

When you want to allow the caller to behave differently based on the specific error that occurred, you can
use an enum to represent the different error cases:

```rust
// An error enum to represent the different error cases
// that may occur when parsing a `u32` from a string.
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}
```

Using an error enum, you're encoding the different error cases in the type system—they become part of the
signature of the fallible function.\
This simplifies error handling for the caller, as they can use a `match` expression to react to the different
error cases:

```rust
match s.parse_u32() {
    Ok(n) => n,
    Err(U32ParseError::Negative) => 0,
    Err(U32ParseError::TooLarge) => u32::MAX,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number: {}", s);
    }
}
```

# Error trait

## Error reporting

In the previous exercise you had to destructure the `TitleError` (see new_alt() on line 237) variant to extract the error message and
pass it to the `panic!` macro. This is a (rudimentary) example of **error reporting**: transforming an error type into a representation that can be shown to a user, a service operator, or a developer.

It's not practical for each Rust developer to come up with their own error reporting strategy: it'd be a waste of time
and it wouldn't compose well across projects.
That's why Rust provides the `std::error::Error` trait.

## The `Error` trait

There are no constraints on the type of the `Err` variant in a `Result`, but it's a good practice to use a type
that implements the `Error` trait.
`Error` is the cornerstone of Rust's error handling story:

```rust
// Slightly simplified definition of the `Error` trait
pub trait Error: Debug + Display {}
```

You might recall the `:` syntax from [the `From` trait](../04_traits/09_from.md#supertrait--subtrait)—it's used to specify **supertraits**.
For `Error`, there are two supertraits: `Debug` and `Display`. If a type wants to implement `Error`, it must also
implement `Debug` and `Display`.

## `Display` and `Debug`

We've already encountered the `Debug` trait in [a previous exercise](../04_traits/04_derive.md)—it's the trait used by
`assert_eq!` to display the values of the variables it's comparing when the assertion fails.

From a "mechanical" perspective, `Display` and `Debug` are identical—they encode how a type should be converted
into a string-like representation:

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

The difference is in their _purpose_: `Display` returns a representation that's meant for "end-users",
while `Debug` provides a low-level representation that's more suitable to developers and service operators.\
That's why `Debug` can be automatically implemented using the `#[derive(Debug)]` attribute, while `Display`
**requires** a manual implementation.

EVERY ERROR ENUM HAVE TO IMLPEMENT ERROR TRAIT.
See TicketNewError on line 188.

# `thiserror`

## Custom error types

We've seen how to implement the `Error` trait "manually" for a custom error type.\
Imagine that you have to do this for most error types in your codebase. That's a lot of boilerplate, isn't it?

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

## You can write your own macros

All the `derive` macros we've seen so far were provided by the Rust standard library.\
`thiserror::Error` is the first example of a **third-party** `derive` macro.

`derive` macros are a subset of **procedural macros**, a way to generate Rust code at compile time.
We won't get into the details of how to write a procedural macro in this course, but it's important
to know that you can write your own!\
A topic to approach in a more advanced Rust course.

## Custom syntax

Each procedural macro can define its own syntax, which is usually explained in the crate's documentation.
In the case of `thiserror`, we have:

- `#[derive(thiserror::Error)]`: this is the syntax to derive the `Error` trait for a custom error type, helped by `thiserror`.
- `#[error("{0}")]`: this is the syntax to define a `Display` implementation for each variant of the custom error type.
  `{0}` is replaced by the zero-th field of the variant (`String`, in this case) when the error is displayed.

# `From` and `Into`

What `.into()` is doing here?

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```

This is the signature of the `new` method:

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

We've also seen that string literals (such as `"A title"`) are of type `&str`.
We have a type mismatch here: a `String` is expected, but we have a `&str`.
No magical coercion will come to save us this time; we need **to perform a conversion**.

## `From` and `Into`

The Rust standard library defines two traits for **infallible conversions**: `From` and `Into`,
in the `std::convert` module.

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

These trait definitions showcase a few concepts that we haven't seen before: **supertraits** and **implicit trait bounds**.
Let's unpack those first.

### Supertrait / Subtrait

The `From: Sized` syntax implies that `From` is a **subtrait** of `Sized`: any type that
implements `From` must also implement `Sized`.
Alternatively, you could say that `Sized` is a **supertrait** of `From`.

### Implicit trait bounds

Every time you have a generic type parameter, the compiler implicitly assumes that it's `Sized`.

For example:

```rust
pub struct Foo<T> {
    inner: T,
}
```

is actually equivalent to:

```rust
pub struct Foo<T: Sized> 
{
    inner: T,
}
```

In the case of `From<T>`, the trait definition is equivalent to:

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

In other words, _both_ `T` and the type implementing `From<T>` must be `Sized`, even
though the former bound is implicit.

### Negative trait bounds

You can opt out of the implicit `Sized` bound with a **negative trait bound**:

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            This is a negative trait bound
    inner: T,
}
```

This syntax reads as "`T` may or may not be `Sized`", and it allows you to
bind `T` to a DST (e.g. `Foo<str>`). It is a special case, though: negative trait bounds are exclusive to `Sized`,
you can't use them with other traits.

## `&str` to `String`

In [`std`'s documentation](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)
you can see which `std` types implement the `From` trait.\
You'll find that `String` implements `From<&str> for String`. Thus, we can write:

```rust
let title = String::from("A title");
```

We've been primarily using `.into()`, though.\
If you check out the [implementors of `Into`](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)
you won't find `Into<String> for &str`. What's going on?

`From` and `Into` are **dual traits**.\
In particular, `Into` is implemented for any type that implements `From` using a **blanket implementation**:

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

If a type `U` implements `From<T>`, then `Into<U> for T` is automatically implemented. That's why
we can write `let title = "A title".into();`.

## `.into()`

Every time you see `.into()`, you're witnessing a conversion between types.\
What's the target type, though?

In most cases, the target type is either:

- Specified by the signature of a function/method (e.g. `Ticket::new` in our example above)
- Specified in the variable declaration with a type annotation (e.g. `let title: String = "A title".into();`)

`.into()` will work out of the box as long as the compiler can infer the target type from the context without ambiguity.

# `TryFrom` and `TryInto`

In the previous chapter we looked at the [`From` and `Into` traits](../04_traits/09_from.md),
Rust's idiomatic interfaces for **infallible** type conversions.\
But what if the conversion is not guaranteed to succeed?

We now know enough about errors to discuss the **fallible** counterparts of `From` and `Into`:
`TryFrom` and `TryInto`.

## `TryFrom` and `TryInto`

Both `TryFrom` and `TryInto` are defined in the `std::convert` module, just like `From` and `Into`.

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

The main difference between `From`/`Into` and `TryFrom`/`TryInto` is that the latter return a `Result` type.\
This allows the conversion to fail, returning an error instead of panicking.

## `Self::Error`

Both `TryFrom` and `TryInto` have an associated `Error` type.
This allows each implementation to specify its own error type, ideally the most appropriate for the conversion
being attempted.

`Self::Error` is a way to refer to the `Error` associated type defined in the trait itself.

## Duality

Just like `From` and `Into`, `TryFrom` and `TryInto` are dual traits.\
If you implement `TryFrom` for a type, you get `TryInto` for free.
