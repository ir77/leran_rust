fn main() {
    println!("Hello, world!");
    let x = 'x';
    // let x2 = 'xx'; // コンパイルエラー
    let two_hearts = '💕';
    let y = "🐢";
    println!("{} {} {}", x, two_hearts, y);

    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    // println!("The second name is: {}", names[3]); // コンパイルエラーで検出できる

    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // aに含まれる全ての要素を持つスライス
    let middle = &a[1..4]; // 1、2、3のみを要素に持つaのスライス
    println!("{}", complete[0]);
    println!("{}", middle[0]);

    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);

    let mut tuple = (1, 2); // x: (i32, i32)
    let other = (2, 3); // y: (i32, i32)

    tuple = other;
    println!("{}", tuple.0);
    println!("{}", tuple.1);
}
