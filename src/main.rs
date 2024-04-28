use std::ops::Add;

#[derive(Debug,Clone, Copy,PartialEq)]
struct Point{
    x:i64,
    y:i64
}

struct Millimeter(u64);
struct Meter(u64);

impl Add<Meter> for Millimeter {
    type Output = Millimeter;
    fn add(self, other: Meter) -> Millimeter {
        Millimeter(self.0 + (other.0*1000))
    }

}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("I'm the pilot and I'm talking to you")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Flyiiiiiiiiing!")
    }
}

impl Human {
    fn fly(&self){
        println!("*make gesture with arms as it can fly*")
    }
}


fn use_traits_with_human() {
    let guy = Human;
    guy.fly();
    Pilot::fly(&guy);
    Wizard::fly(&guy);
}



fn main() {
    use_traits_with_human()
}
