use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

/*
    Closure might:
    FnOnce      taking ownership
    FnMut       borrowing mutably
    Fn          borrowing immutably

    Once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined (thus affecting what, if anything, is moved into the closure),
    the code in the body of the closure defines what happens to the references or values when the closure is evaluated later (thus affecting what, if anything, is moved out of the closure). 
    A closure body can do any of the following: move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment to begin with.

    The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. 
    Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
    
    FnOnce       applies to closures that can be called ONCE. All closures implement AT LEAST this trait because all closures can be called. 
                A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.


    FnMut        applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
    Fn           applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment.
                These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times CONCURRENTLY.

*/

fn capturing_works_only_for_closures() {
    let x = 4;
    let equal_to_x = |z| z == x; // Fn example of capturing
    let y = 4;
    assert!(equal_to_x(y));
}

/*
fn capturing_not_even_compiles_for_functions() {
    let x = 4;
    fn equal_to_x(z: i32) -> bool { z == x } // can't capture dynamic environment in a fn item
    let y = 4;
    assert!(equal_to_x(y));
}
*/

// FnOnce example of capturing
fn capturing_to_a_new_thread() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // FnOnce example of capturing
    // println!("невозможно использовать x здесь: {:?}", x); // Doesn't compile! 
    
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    
    //assert!(equal_to_x(y)); // Do not compile
    //assert!(equal_to_x(y.clone())); // Do not compile
}

fn main() {

    fn add_one_v1    (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // Compile error: type must be known at this points
    //let add_one_v3 = |y|             { y + 1 };
    //let add_one_v4 = |z|               z + 1 ;

    // Compile error: types cannot be inferred
    //let example_closure = |x| x;
    //let s = example_closure(String::from("Привет"));
    //let n = example_closure(5);

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    generate_workout2(
        simulated_user_specified_value,
        simulated_random_number
    );

    generate_workout3(
        simulated_user_specified_value,
        simulated_random_number
    );

    generate_workout4(
        simulated_user_specified_value,
        simulated_random_number
    );

    generate_workout5(
        simulated_user_specified_value,
        simulated_random_number
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/*
Рассмотрим  гипотетическую  ситуацию:  мы  работаем  в  стартапе,  где  создается 
приложение для генерирования индивидуальных планов тренировок. Бэкенд на­
писан на Rust, и алгоритм, который генерирует план тренировок, берет в расчет 
многие факторы, такие как возраст пользователя приложения, индекс массы тела, 
предпочтения в упражнениях, последние тренировки и интенсивность, задавае­
мую пользователем. Фактический алгоритм, используемый в примере, не важен. 
Важно то, что этот расчет занимает несколько секунд. Мы хотим вызывать этот 
алгоритм, только когда нужно и лишь один раз, чтобы не заставлять пользователя 
ждать больше, чем необходимо.
*/
////////////////////////////////////////////////////////////////////////////////////////////////////

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("1 вычисляется медленно...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("1 Сегодня сделайте {} отжиманий!", simulated_expensive_calculation(intensity)
        );
        println!("1 Далее, сделайте {} приседаний!", simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("1 Сделайте сегодня перерыв! Пейте больше воды!");
        } else {
            println!("1 Сегодня пробежка {} минут!", simulated_expensive_calculation(intensity));
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/*
let expensive_closure = |num| {
    println!("вычисляется медленно...");
    thread::sleep(Duration::from_secs(2));
    num
};

// Actually is (but optional);
let expensive_closure = |num: u32| -> u32 {
    println!("вычисляется медленно...");
    thread::sleep(Duration::from_secs(2));
    num
};

*/
fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("2 вычисляется медленно...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("2 Сегодня сделайте {} отжиманий!", expensive_closure(intensity));
        println!("2 Далее, сделайте {} приседаний!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("2 Сделайте сегодня перерыв! Пейте больше воды!");
        } else {
            println!("2 Сегодня пробежка {} минут!", expensive_closure(intensity));
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// Optimize executing of closers in generate_workout2 by caching results,
// to make sure we don't really call for the same result twice.
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
  where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg); // Call closure once
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout3(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("3 вычисляется медленно...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("3 Сегодня сделайте {} отжиманий!", expensive_result.value(intensity));
        println!("3 Далее, сделайте {} приседаний!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("3 Сделайте сегодня перерыв! Пейте больше воды!");
        } else {
            println!("3 Сегодня пробежка {} минут!", expensive_result.value(intensity));
        }
    }
}

#[test]
#[should_panic]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// Make Cacher to pass the test call_with_different_values2

struct Cacher1<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    results: HashMap<u32, u32>
}

impl<T> Cacher1<T>
  where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher1<T> {
        Cacher1 {
            calculation,
            results: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {

        let value = self.results.entry(arg).or_insert((self.calculation)(arg));
        *value
    }
}

fn generate_workout4(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher1::new(|num| {
        println!("4 вычисляется медленно...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("4 Сегодня сделайте {} отжиманий!", expensive_result.value(intensity));
        println!("4 Далее, сделайте {} приседаний!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("4 Сделайте сегодня перерыв! Пейте больше воды!");
        } else {
            println!("4 Сегодня пробежка {} минут!", expensive_result.value(intensity));
        }
    }
}

#[test]
fn call_with_different_values2() {
    let mut c = Cacher1::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// Make Cacher to pass the test call_with_different_values3 AND be GENERIC

struct Cacher2<T, K, V>
    where T: Fn(&K) -> V,
        K: Display + PartialEq + Eq + Hash + Clone,
        V: Display + Clone,
{
    calculation: T,
    results: HashMap<K, V>
}

impl<T, K, V> Cacher2<T, K, V>
  where T: Fn(&K) -> V,
        K: Display + PartialEq + Eq + Hash + Clone,
        V: Display + Clone,
{
    fn new(calculation: T) -> Cacher2<T, K, V> {
        Cacher2 {
            calculation,
            results: HashMap::new()
        }
    }

    fn value(&mut self, arg: K) -> V {

        self.results
            .entry(arg.clone())
            .or_insert_with(|| (self.calculation)(&arg)).clone()
    }
}

fn generate_workout5(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher2::new(|num : &u32| {
        println!("5 вычисляется медленно...");
        thread::sleep(Duration::from_secs(2));
        num.clone() //*num
    });

    if intensity < 25 {
        println!("5 Сегодня сделайте {} отжиманий!", expensive_result.value(intensity));
        println!("5 Далее, сделайте {} приседаний!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("5 Сделайте сегодня перерыв! Пейте больше воды!");
        } else {
            println!("5 Сегодня пробежка {} минут!", expensive_result.value(intensity));
        }
    }
}

#[test]
fn call_with_different_values3() {
    
    let mut c = Cacher2::new(|a: &u32| a.clone());
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}


#[test]
fn call_with_string_type() {
    
    let mut cacher = Cacher2::new(|s: &String| s.len());
    let result = cacher.value("hello".to_string());
    assert_eq!(result, 5);
}
