use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
    let circle = Circle { x: 0.0, y: 0.0, radius: 10.0 };
    println!("{}", circle.area());
    print_area(circle);

    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };
    assert!(r.is_square());
    r.height = 42;
    assert!(!r.is_square());

    foo("aaa");
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// traitを用いて実装することでジェネリクス関数が使いやすくなる
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}
impl<T: PartialEq> Rectangle<T> { // publicなEquatableもある
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn foo<T: Clone + Debug>(x: T) { // トレイト境界は+で複数付与することが出来る
    x.clone();
    println!("{:?}", x);
}