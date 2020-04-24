fn main() {
    println!("Hello, world!");
    for x in 0..10 { // C styleのforは存在しない
        println!("{}", x); // x: i32
    }

    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }
}
