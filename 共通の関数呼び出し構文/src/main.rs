fn main() {
    println!("Hello, world!");
    trait Foo {
        fn f(&self);
    }
    trait Bar {
        fn f(&self);
    }
    struct Baz;
    impl Foo for Baz {
        fn f(&self) { println!("Baz’s impl of Foo"); }
    }
    impl Bar for Baz {
        fn f(&self) { println!("Baz’s impl of Bar"); }
    }
    let b = Baz; // 名前がかぶっている関数の場合は以下のように記述する
    Foo::f(&b);
    Bar::f(&b);
}
