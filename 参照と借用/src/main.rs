fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let answer = foo(&v1, &v2);

    println!("{}, {}, {}", answer, v1[0], v2[1]);

    let mut x = 5;
    {
        // 借用は全て所有者のスコープより長く存続してはならない
        let y = &mut x;
        *y += 1;
        x += 1;
    }
    println!("{}", x);

    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        // vは借用されているのでここでvの変更はできない
    }
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // v1とv2についての作業を行う
    // 答えを返す
    42
}
