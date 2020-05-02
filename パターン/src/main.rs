fn main() {
    println!("Hello, world!");
    let x = 'x';
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c), // xという新しい束縛を導入している（上で宣言したxとは別）
    }

    match c {
        'x' => println!("x: {} c: {}", x, c),
        _ => println!(""),
    }

    println!("x: {}", x);

    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // または、が可能
        3 => println!("three"),
        _ => println!("anything"),
    }
    match x {
        1 ..= 5 => println!("one through five"),
        _ => println!("anything"),
    }
    let x = '💅';
    match x {
        'a' ..= 'j' => println!("early letter"),
        'k' ..= 'z' => println!("late letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    
    let origin = Point { x: 0, y: 0 };

    // 構造体の分解も可能
    match origin {
        Point { x, y } => println!("({},{})", x, y), 
    }

    // 構造体の分解時に名前を付けることも可能
    match origin {
        Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
    }

    // 構造体の一部の値だけ取り出すのも可能
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    match origin {
        Point { y, .. } => println!("y is {}", y),
    }

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }

    fn coordinate() -> (i32, i32, i32) {
        // 3要素のタプルを生成して返す
        (1, 1, 1)
    }
    // _で無視するパターン
    let (x, _, z) = coordinate();
    
    // ..で値を無視するパターン
    let x = OptionalTuple::Value(5, -2, 3);
    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }

    // 参照を取得することも可能
    let mut x = 5;
    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    let x = 1;
    match x {
        // @でeに束縛が可能
        e @ 1 ..= 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }
    
    let x = OptionalInt::Value(5);
    
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}
