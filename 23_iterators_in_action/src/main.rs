fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // Nothing yet happened here. It's Lazy.

    for val in v1_iter { // FOR implicitly takes ownership of 
                         // v1_iter and made it mutable behind the scenes.
        println!("Got: {val}");
    }

    // Same but manually:
    let mut v2_iter = v1.iter(); // If we want to use NEXT() manually
                                 // then we need to make sure it is MUTABLE
                                 // as every NEXT() consumes, or uses up iterator,
                                 // changes internal state that the iterator 
                                 // to keep track of where it is in the sequence.
    assert_eq!(v2_iter.next(), 
        Some(&1)); // Immutable references!!!
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    // If we need to iterate over mutable references:
    let mut v2 = vec![1, 2, 3];
    let mut v3_iter = v2.iter_mut(); // Iterator with mutable items.
    assert_eq!(v3_iter.next(), 
        Some(&mut 1)); // Mutable references!!!
    assert_eq!(v3_iter.next(), Some(&mut 2));
    assert_eq!(v3_iter.next(), Some(&mut 3));
    assert_eq!(v3_iter.next(), None);

    // Create an iterator that takes ownership of vector and returns owned values
    // If we need to iterate over OWNED (take ownership!) items:
    let mut v3 = vec![1, 2, 3];
    let mut v4_iter = v3.into_iter(); // Iterator to take ownership over items.
    assert_eq!(v4_iter.next(), 
        Some(1)); // Mutable references!!!
    assert_eq!(v4_iter.next(), Some(2));
    assert_eq!(v4_iter.next(), Some(3));
    assert_eq!(v4_iter.next(), None);
}

#[test]
fn iterator_sum_consumes_iterator() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // We aren’t allowed to use v1_iter after 
    // the call to sum because sum takes ownership of the iterator we call it on.

    assert_eq!(total, 6);
}

// Iterator adapters are methods defined on the Iterator trait that don’t consume the iterator. 
// Instead, they produce different iterators by changing some aspect of the original iterator.
#[test]
fn iterator_map_is_iterator_adapter() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // Many iterator adapters take closures as arguments, and commonly 
    // the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // The shoes_in_size function takes ownership of a vector of shoes and a shoe size as parameters. 
    // It returns a vector containing only shoes of the specified size.
    shoes
        .into_iter() // create an iterator that takes ownership of the vector
        .filter(|s| s.size == shoe_size) // adapt that iterator into a new iterator that only contains elements for which the closure returns true.
        .collect() // gather the values returned by the adapted iterator into a vector that’s returned by the function
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

///////////// Let's implement our own Iterator that counts from 1 to 5 ////
// Метод next — единственный, для которого требуется определение. 
// После того как вы его предоставите, вы смо­жете использовать все другие методы 
// с реализациями по умолчанию, предусмо­тренные типажом Iterator!

struct Counter {
    count: u32, // Это поле содержит значение типа u32, которое будет отслеживать,
    // где мы находимся в процессе перебора зна­чений от 1 до 5.
    // Поле count является приватным, потому что мы хотим, чтобы 
    // реализация структуры Counter управляла своим значением.
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Превратим Counter в итератор, реализовав типаж Iterator.
impl Iterator for Counter {
    type Item = u32; // В итераторе мы устанавливаем связанный тип Item равным u32, 
                     // то есть итератор будет возвращать значения типа u32.
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// Мы  реализовали  типаж  Iterator  путем  определения  метода  next,  
// поэтому  те­перь мы можем использовать реализации по умолчанию любого метода 
// типажа Iterator, определенного в стандартной библиотеке, поскольку все они 
// использу­ют функциональность метода next.
// Например, если по какой­то причине мы хотим взять значения, произведенные 
// экземпляром структуры Counter, соединить их в пары со значениями, 
// произве­денными еще одним экземпляром структуры Counter после пропуска 
// первого зна­чения, перемножить пары между собой, сохранить только те результаты, 
// которые делятся на 3, и сложить все полученные значения вместе, то 
// мы можем сделать это, как показано в тесте из листинга 13.23.
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // Метод zip производит только четыре пары. 
        // Теоретически возможная пятая пара (5, None) никогда не производится, 
        // потому что zip возвра­щает None, когда любой из его входных итераторов возвращает None.
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

// Итераторы можно создавать из других типов коллекций 
// стандартной библиотеки, таких как HashMap. 