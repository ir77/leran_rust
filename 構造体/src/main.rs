struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

// struct Color(i32, i32, i32); // タプル構造体で簡易に構造体を宣言することができるが、基本的には構造体を使ったほうが参照しやすい

fn main() {
    let mut point = Point { x: 0, y: 0 };
    point.x = 5;
    let point = point; // この変更でイミュータブルにできる
    println!("The point is at ({}, {})", point.x, point.y);

    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, .. point }; // 値の一部を他の構造体からコピーできる
    println!("{}, {}, {}", point.x, point.y, point.z);

    struct Inches(i32); // タプル構造体が1つの値だけの場合、他の要素と区別できるメリットがある（Swiftのaliasのようなイメージか？）
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}