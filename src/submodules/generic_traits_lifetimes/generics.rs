fn pseudo_main() {
    let mut list_1 = vec![34, 69, 21, 89, 2, 99, 36, 34];
    let mut list_2: Vec<i64> = vec![34, 50, 25, 100, 65];
    let mut char_list = vec!['y', 'm', 'a', 'q'];
    let mut largest_num: &i32 = largest(&list_1);
    let mut largest_car = largest(&char_list);

    println!("{}", largest_num);
    println!("{}", largest_car);

    let integer = Point { x: 23, y: 77 };
    let float = Point { x: 9.6, y: 7.85 };

    println!("x value is: {}", integer.x());
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_number = &list[0];
    for number in list {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}

// In Struct Definitions && In Method Definitions - Start
struct Point<T> {
    x: T,
    y: T,
}

// By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
// Methods written within an impl that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// In Struct Definitions && In Method Definitions - END

// In Enum Definitions
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
