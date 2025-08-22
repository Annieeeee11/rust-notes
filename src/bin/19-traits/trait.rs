trait Shape {
    //traits -- everything thatis here should be peresnt where we implement this
    fn area(&self) -> u32;
}

// struct for implement
struct Rect {
    width: u32,
    height: u32,
}
impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}
struct Circle {
    round: u32,
}
impl Shape for Circle {
    fn area(&self) -> u32 {
        return self.round * self.round;
    }
}


fn main() {
    let r = Rect {
        width: 10,
        height: 20,
    };
    let c = Circle { round: 20 };
    get_area(c);
    get_area(r);
}

// get_area is a function it takes s as input -- s is generic type T -- which is a bound to a trait shape
fn get_area<T: Shape>(s: T) -> u32 {
    return s.area();
}
