fn main() {
    println!("Hello, world!");

    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw }; // 生ポインタを参照外しする際はunsafeが必要
    println!("raw points at {}", points_at);
}
