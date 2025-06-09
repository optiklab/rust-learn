use std::fmt::Display;

// Because lifetimes are a type of generic, the declarations of the lifetime 
// parameter 'a and the generic type parameter T go in the same list inside 
// the angle brackets after the function name.
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

// Lifetime names always need to be used after the struct’s name 
// because those lifetimes are part of the struct’s type.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime names for struct fields always need to be declared after the impl 
// keyword and then used after the struct’s name because those lifetimes 
// are part of the struct’s type.
impl<'a> ImportantExcerpt<'a> {
    // The lifetime elision rules often make it so that lifetime annotations 
    // aren’t necessary in method signatures. 
    fn level(&self) -> i32 {
        // Not required to annotate the lifetime of the reference to self 
        // because of the first elision rule.
        3
    }

    // There are two input lifetimes: self & announcement.
    // Rust applies lifetime elision rule #1 and gives both &self and announcement their own lifetimes. 
    // Then, rule #3: because one of the parameters is &self, the return type gets the lifetime of &self, 
    // and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {result}");

    let string3 = String::from("long string is long");
    let result2;
    {
        let string4 = String::from("xyz");
        result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is: {result2}");
    }
    // Will not compile: string4 borrowed value does not live long enough.
    //println!("The longest string is {result2}");

    let string5 = String::from("abcd");
    let string6 = "efghijklmnopqrstuvwxyz";
    let result3 = always_first(string5.as_str(), string6);
    println!("Always _first string is: {result3}");

    //let x: i32 = 5;
    //let y: i32 = 10;
    //let r: &i32 = &x; // r is a reference to x
    //let re: &'a i32 = &x; // re is a reference with an explicit lifetime 'a    
    //let mre: &'a mut i32 = &mut y; // mutable reference with an explicit lifetime 'a

    let novel = String::from("Call me Anton. Some years ago...");
    println!("{novel}");
    let first_sentence = novel.split('.')
        .next() // " Call me Anton."
        .unwrap(); // Convert Option<&str> to &str
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {}", i.part);
    // The ImportantExcerpt struct has a lifetime parameter 'a

    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {s}");
}

/*
fn longest(x: &str, y: &str) -> &str {
              ----     ----     ^ expected named lifetime parameter
  this function's return type contains a borrowed value, 
  but the signature does not say whether it is borrowed from `x` or `y`
  help: consider introducing a named lifetime parameter
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// We’ve specified a lifetime parameter 'a for the parameter x and 
// the return type, but not for the parameter y, because the lifetime of y 
// does not have any relationship with the lifetime of x or the return value.
fn always_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Will not compile because the return value lifetime is not related
// to the lifetime of the parameters at all. 
/*
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

// An actual function declaration is looking like this:
//fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &String) -> &str {
// but we skip and shorten it due to #2 rule of Lifetime Elision Rules of Rust 
// (if there is exactly one input lifetime parameter, 
// that lifetime is assigned to all output lifetime parameters.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
The patterns programmed into Rust’s analysis of references are called 

    the lifetime elision rules. 

These are a set of particular cases that the compiler will consider, 
and if your code fits these cases, you don’t need to write the lifetimes explicitly.

Lifetimes on function or method parameters are called input lifetimes, 
and lifetimes on return values are called output lifetimes.

The compiler uses 3 rules to figure out the lifetimes of the references when 
there aren’t explicit annotations. The first rule applies to input lifetimes, 
and the second and third rules apply to output lifetimes. If the compiler gets 
to the end of the three rules and there are still references for which it can’t 
figure out lifetimes, the compiler will stop with an error that you can resolve 
by adding the lifetime annotations. These rules apply to fn definitions 
as well as impl blocks.

#1 rule: compiler assigns a lifetime parameter to each parameter that’s a reference. 
    In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); 
    a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 
    and so on.

#2 rule: if there is exactly one input lifetime parameter, 
    that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

#3 rule: if there are multiple input lifetime parameters, but one of them is 
    &self or &mut self because this is a method, the lifetime of self is assigned 
    to all output lifetime parameters. This third rule makes methods much nicer 
    to read and write because fewer symbols are necessary.

*/