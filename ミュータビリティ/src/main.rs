fn main() {
    println!("Hello, world!");
    let mut x = 5;
    let y = &mut x;
    println!("{}", y);
    *y = 10; // y = はできないがmutが入っているので中身を変えることはできる
    println!("{}", y);

    let mut a = Point { x: 5 };
    a.x = 10;
    println!("{}", a.x);
}

struct Point {
    x: i32,
    // mut y: i32, // ダメ
}