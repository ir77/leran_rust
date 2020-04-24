fn main() {
    println!("Hello, world!");
    let x = 5;

    if x == 5 {
        println!("x は 5 です!");
    } else {
        println!("x は 5 ではありません :(");
    }

    // if は式だから以下のように書くことができる
    let y = if x == 5 { 10 } else { 15 };
    println!("{}", y);
}
