fn main() {
    println!("Hello, world!");
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_works2() {
        // panic!()
        assert!(false);
    }
}
