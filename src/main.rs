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

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}
