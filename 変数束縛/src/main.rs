fn main() {
    let (x, y) = (1, 2);
    println!("Hello, world! {} {}", x, y);

    let x: i32 = 8;
    {
        println!("{}", x); // "8"を印字する
        let x = 12;
        println!("{}", x); // "12"を印字する
    }
    println!("{}", x); // "8"を印字する
    let x =  42;
    println!("{}", x); // "42"を印字する
}
