fn main() {
    println!("Hello, world!");
    let nums = (1..100).collect::<Vec<_>>();
    for num in nums {
        println!("{}", num);
    }
}
