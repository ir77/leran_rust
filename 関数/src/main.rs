fn main() {
    println!("Hello, world!");

    let number = add_one(1);
    print_number(number);

    /*
    文: 値を返さない, 文は2種類（宣言文, 式文）しかない
        宣言文: let は文の先頭にしかなれない
        式文: 式を文に変換すること。;を付けたもの
    式: 値を返す, 文以外のもの
    */

    let mut y = 5;
    let x = (y = 6);  // xは値 `()` を持っており、 `6` ではありません
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

