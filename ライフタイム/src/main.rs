struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { 
        self.x 
    }
}

// struct Bar {
//     x: & i32,
// }

fn main() {
    let y = &5; // これは`let _y = 5; let y = &_y;`と同じ
    let f = Foo { x: y };

    println!("{}", f.x());
}