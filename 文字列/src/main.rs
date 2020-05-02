fn main() {
    println!("Hello, world!");
    // Rustには主要な文字列型が二種類あります。&str と Stringです
    // &str は「文字列スライス」と呼ばれます。 文字列スライスは固定サイズで変更不可能です
    let greeting = "Hello there."; // greeting: &'static str
    println!("{}", greeting);

    let s = "foo
    bar";
    assert_eq!("foo\n    bar", s);

    // String というヒープアロケートされる文字列もあります。 この文字列は伸張可能であり、またUTF-8であることも保証されています
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    // String は & によって &str に型強制されます。
    fn takes_slice(slice: &str) {
        println!("Got: {}", slice);
    }
    let s = "Hello".to_string();
    takes_slice(&s);

    let hachiko = "忠犬ハチ公";
    // println!("The first letter of s is {}", s[0]); // エラー!!! このアクセスは許容していない
    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }
    println!("");
    for c in hachiko.chars() {
        print!("{}, ", c);
    }
    println!("");

    let hello = "Hello ".to_string();
    let world = "world!";
    let hello_world = hello + world; // Stringに&strは連結可能

    let hello = "Hello ".to_string();
    let world = "world!".to_string();
    let hello_world = hello + &world; // String同士は連結できないので&を付けて&strに型強制してから連結する
}
