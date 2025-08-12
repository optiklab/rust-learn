
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

# `Error::source`

There's one more thing we need to talk about to complete our coverage of the `Error` trait: the `source` method.

```rust
// Full definition this time!
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

The `source` method is a way to access the **error cause**, if any.\
Errors are often chained, meaning that one error is the cause of another: you have a high-level error (e.g.
cannot connect to the database) that is caused by a lower-level error (e.g. can't resolve the database hostname).
The `source` method allows you to "walk" the full chain of errors, often used when capturing error context in logs.

## Implementing `source`

The `Error` trait provides a default implementation that always returns `None` (i.e. no underlying cause). That's why
you didn't have to care about `source` in the previous exercises.\
You can override this default implementation to provide a cause for your error type.

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

In this example, `DatabaseError` wraps an `std::io::Error` as its source.
We then override the `source` method to return this source when called.

## `&(dyn Error + 'static)`

What's this `&(dyn Error + 'static)` type?\
Let's unpack it:

- `dyn Error` is a **trait object**. It's a way to refer to any type that implements the `Error` trait.
- `'static` is a special **lifetime specifier**.
  `'static` implies that the reference is valid for "as long as we need it", i.e. the entire program execution.

Combined: `&(dyn Error + 'static)` is a reference to a trait object that implements the `Error` trait
and is valid for the entire program execution.

Don't worry too much about either of these concepts for now. We'll cover them in more detail in future chapters.

## Implementing `source` using `thiserror`

`thiserror` provides three ways to automatically implement `source` for your error types:

- A field named `source` will automatically be used as the source of the error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- A field annotated with the `#[source]` attribute will automatically be used as the source of the error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- A field annotated with the `#[from]` attribute will automatically be used as the source of the error **and**
  `thiserror` will automatically generate a `From` implementation to convert the annotated type into your error type.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## The `?` operator

The `?` operator is a shorthand for propagating errors.\
When used in a function that returns a `Result`, it will return early with an error if the `Result` is `Err`.

For example:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

is equivalent to:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

You can use the `?` operator to shorten your error handling code significantly.\
In particular, the `?` operator will automatically convert the error type of the fallible operation into the error type
of the function, if a conversion is possible (i.e. if there is a suitable `From` implementation)
