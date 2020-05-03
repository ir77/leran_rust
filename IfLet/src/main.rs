fn main() {
    println!("Hello, world!");
    let number: Option<i32> = Some(5);
    if number.is_some() {
        println!("{}", number.unwrap());
    }
    if let Some(number) = number { // 上よりこっちのほうが良い。極力強制unwrapはしたくない。
        println!("{}", number);
    }
}
