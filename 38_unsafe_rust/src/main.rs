use std::slice;

fn main() {
    println!("Hello, world!");

    let mut num = 5;

    // Creating raw pointers with the raw borrow operators. No need for unsafe
    // keyword. We can create raw pointers in safe code...
    let r1 = &raw const num; // *const i32 immutable raw pointer
    let r2 = &raw mut num; // *mut i32 mutable raw pointer

    // But we can’t dereference raw pointers outside an unsafe block:
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // We can create a raw pointer whose validity we can’t be so certain of, 
    // using as to cast a value instead of using the raw borrow operators. 
    // Trying to use arbitrary memory is undefined: there might be data 
    // at that address or there might not, the compiler might optimize the code 
    // so there is no memory access, 
    // or the program might terminate with a segmentation fault. 
    // Usually, there is no good reason to write code like this, especially in 
    // cases where you can use a raw borrow operator instead, but it is possible.
    let address = 0x012345usize;
    let r = address as *const i32;
    // Most likely crash:
    //let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    read_union_fields();
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // You can't avoid unsafe code here. This will not compile
    // Error[E0499]: cannot borrow `*values` as mutable more than once at a time
    //(&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

////////////////////////////// Unsafe trait ////////////////////////////////////
unsafe trait Foo {
    // methods go here

    // A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// As an example, recall the Sync and Send marker traits we discussed in 
// [“Extensible Concurrency with the Sync and Send Traits” in Chapter 16](https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits).
// the compiler implements these traits automatically if our types are composed 
// entirely of other types that implement Send and Sync. If we implement a type 
// that contains a type that does not implement Send or Sync, such as raw pointers, 
// and we want to mark that type as Send or Sync, we must use unsafe. 
// Rust can’t verify that our type upholds the guarantees that it can be safely
// sent across threads or accessed from multiple threads; therefore, we need to 
// do those checks manually and indicate as such with unsafe.
// https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html

////////////////////////// Accessing Fields of a Union /////////////////////////

// The final action that works only with unsafe is accessing fields of a union. 
// A union is similar to a struct, but only one declared field is used in a 
// particular instance at one time. Unions are primarily used to interface with 
// unions in C code. Accessing union fields is unsafe because Rust can’t guarantee 
// the type of the data currently being stored in the union instance. 
// You can learn more about unions in the [Rust Reference](https://doc.rust-lang.org/stable/reference/items/unions.html).

union MyUnion {
    f1: u32,
    f2: f32,
}

fn read_union_fields() {
    let u = MyUnion { f1: 1 };

    unsafe {
        let f = u.f1;
        println!("Field1: {f}");
    }

    f(u);
}

fn f(u: MyUnion) {
    unsafe {
        match u {
            MyUnion { f1: 1 } => { println!("one"); }
            MyUnion { f1: 10 } => { println!("ten"); }
            MyUnion { f2 } => { println!("{}", f2); }
        }
    }
}