struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) { // teardownやdeferのようなことが出来る
        println!("Dropping!");
    }
}

fn main() {
    println!("start");
    let x = HasDrop;

    // いくつかの処理
    println!("end");
} // x はここでスコープ外になります