# Enumerations

Based on the validation logic you wrote [in a previous chapter](../03_ticket_v1/02_validation.md),
there are only a few valid statuses for a ticket: `To-Do`, `InProgress` and `Done`.\
This is not obvious if we look at the `status` field in the `Ticket` struct or at the type of the `status`
parameter in the `new` method:

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

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

In both cases we're using `String` to represent the `status` field.
`String` is a very general type—it doesn't immediately convey the information that the `status` field
has a limited set of possible values. Even worse, the caller of `Ticket::new` will only find out **at runtime**
if the status they provided is valid or not.

We can do better than that with **enumerations**.

## `enum`

An enumeration is a type that can have a fixed set of values, called **variants**.\
In Rust, you define an enumeration using the `enum` keyword:

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum`, just like `struct`, defines **a new Rust type**.


# `match`

You may be wondering—what can you actually **do** with an enum?\
The most common operation is to **match** on it.

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // The `|` operator lets you match multiple patterns.
            // It reads as "either `Status::ToDo` or `Status::InProgress`".
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

A `match` statement that lets you compare a Rust value against a series of **patterns**.\
You can think of it as a type-level `if`. If `status` is a `Done` variant, execute the first block;
if it's a `InProgress` or `ToDo` variant, execute the second block.

## Exhaustiveness

There's one key detail here: `match` is **exhaustive**. You must handle all enum variants.\
If you forget to handle a variant, Rust will stop you **at compile-time** with an error.

E.g. if we forget to handle the `ToDo` variant:

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

the compiler will complain!

## Catch-all

If you don't care about one or more variants, you can use the `_` pattern as a catch-all:

```rust
match status {
    Status::Done => true,
    _ => false
}
```

The `_` pattern matches anything that wasn't matched by the previous patterns.

By using this catch-all pattern, you _won't_ get the benefits of compiler-driven refactoring.
If you add a new enum variant, the compiler _won't_ tell you that you're not handling it.

If you're keen on correctness, avoid using catch-alls. Leverage the compiler to re-examine all matching sites and determine how new enum variants should be handled.

# Variants can hold data

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Our `Status` enum is what's usually called a **C-style enum**.\
Each variant is a simple label, a bit like a named constant. You can find this kind of enum in many programming
languages, like C, C++, Java, C#, Python, etc.

Rust enums can go further though. We can **attach data to each variant**.

## Variants

Let's say that we want to store the name of the person who's currently working on a ticket.\
We would only have this information if the ticket is in progress. It wouldn't be there for a to-do ticket or
a done ticket.
We can model this by attaching a `String` field to the `InProgress` variant:

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

`InProgress` is now a **struct-like variant**.\
The syntax mirrors, in fact, the one we used to define a struct—it's just "inlined" inside the enum, as a variant.

## Accessing variant data

If we try to access `assigned_to` on a `Status` instance,

```rust
let status: Status = /* */;

// This won't compile
println!("Assigned to: {}", status.assigned_to);
```

the compiler will stop us:

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` is **variant-specific**, it's not available on all `Status` instances.\
To access `assigned_to`, we need to use **pattern matching**:

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## Bindings

In the match pattern `Status::InProgress { assigned_to }`, `assigned_to` is a **binding**.\
We're **destructuring** the `Status::InProgress` variant and binding the `assigned_to` field to
a new variable, also named `assigned_to`.\
If we wanted, we could bind the field to a different variable name:

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

# Concise branching

Your solution to the previous exercise probably looks like this:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```

You only care about the `Status::InProgress` variant.
Do you really need to match on all the other variants?

New constructs to the rescue!

## `if let`

The `if let` construct allows you to match on a single variant of an enum,
without having to handle all the other variants.

Here's how you can use `if let` to simplify the `assigned_to` method:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```

## `let/else`

If the `else` branch is meant to return early (a panic counts as returning early!),
you can use the `let/else` construct:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```

It allows you to assign the destructured variable without incurring
any "right drift", i.e. the variable is assigned at the same indentation level
as the code that precedes it.

## Style

Both `if let` and `let/else` are idiomatic Rust constructs.\
Use them as you see fit to improve the readability of your code,
but don't overdo it: `match` is always there when you need it.
