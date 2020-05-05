fn main() {
    println!("Hello, world!");
}

struct Hoge<T> {
    hoge: i32,
    // foo: [T], // 最後でしか使えない
    bar: [T],
}
