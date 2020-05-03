fn main() {
    println!("Hello, world!");
    let x: Option<i32> = Some(5);
    println!("{}", x.unwrap_or(0));

    struct Point<T> {
        x: T,
        y: T,
    }
    
    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };
}
