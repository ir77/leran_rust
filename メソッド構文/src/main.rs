fn main() {
    println!("Hello, world!");
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    
    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        fn grow(&self, increment: f64) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius + increment }
        }

        fn position(&self) -> (f64, f64) {
            (self.x, self.y)
        }

        fn reference(&self) {
            println!("taking self by reference!");
        }
    
        fn mutable_reference(&mut self) {
        println!("taking self by mutable reference!");
        }
    
        fn takes_ownership(self) {
        println!("taking ownership of self!");
        }
    }
    
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
    println!("{}", c.position().0);

    let d = c.grow(2.0).area();
    println!("{}", d);

    // staticメソッドの定義（rustでは関連関数と呼ぶ）
    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }
    }

    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());

    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
        }
    
        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }
    
        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.y = coordinate;
            self
        }
    
        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
            self.radius = radius;
            self
        }
    
        fn finalize(&self) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius }
        }
    }

    // デフォルト引数などを定義したい場合は↑↓のようにビルダーパターンを用いる
    let c = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}
