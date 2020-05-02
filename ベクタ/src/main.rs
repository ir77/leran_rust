fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32> 動的で拡張可能
    let mut v = vec![0; 10]; // 0が10個
    println!("{}", v[0]); // ここの0はi32ではなくusize型なので注意する

    for i in &v {
        println!("A reference to {}", i);
    }
    
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
