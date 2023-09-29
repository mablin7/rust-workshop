struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}
struct Triangle {
    base: f64,
    height: f64,
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.141592653589793238 * self.radius * self.radius
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

trait AreaSquared {
    fn area_squared(&self) -> f64;
}

impl<T: Area> AreaSquared for T {
    fn area_squared(&self) -> f64 {
        self.area().powf(2.0)
    }
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let square = Square { side: 2.0 };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };

    println!("circle area = {}", circle.area());
    println!("square area = {}", square.area());
    println!("triangle area = {}", triangle.area());

    println!("circle area squared = {}", circle.area_squared());
}
