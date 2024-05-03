use std::ops::Add;
use std::fmt;
use std::fmt::Formatter;

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
#[allow(unused)]
impl Human {
    fn fly(&self){
        println!("*make gesture with arms as it can fly*")
    }
}

#[allow(unused)]
fn use_traits_with_human() {
    let guy = Human;
    guy.fly();
    Pilot::fly(&guy);
    Wizard::fly(&guy);
}

//if no self in method definition :
//<Type as Trait>::function(destinataire_si_methode, argument_suivant, ...);

trait OutlinePrint: fmt::Display{
    fn outline_print(&self){
        let value = self.to_string();
        let width = value.len();
        println!("{}","*".repeat(width+4));
        println!("*{}*", " ".repeat(width + 2));
        println!("* {} *", value);
        println!("*{}*", " ".repeat(width + 2));
        println!("{}", "*".repeat(width + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Envelop(Vec<String>);

impl fmt::Display for Envelop {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn type_object(){
    type Kilometers = i64;
    let x:i64 = 5;
    let y:i64 = 5;
    println!("x + y = {}",x+y)
}



fn main() {
    let w = Envelop(vec![String::from("hey"), String::from("my dear Friend")]);
    println!("w = {:?}", w);
}
