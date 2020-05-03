trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn do_something<T: Foo>(x: T) {
    x.method();
}

fn do_something2(x: &Foo) {
    x.method();
}

fn main() {
    println!("Hello, world!");

    let x = 5u8;
    let y = "Hello".to_string();
    do_something(x);
    do_something(y);

    let x = 5u8;
    do_something2(&x as &Foo); // 動的ディスパッチ。キャストが必要になったり最適化の機会を阻害する可能性あり
}
