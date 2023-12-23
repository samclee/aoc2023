use std::io::Result;
use std::collections::HashSet;
use std::ops::{Add,Sub};

#[cfg(test)]
mod tests;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct MyVec {
    x: i32,
    y: i32
}

impl MyVec {
    fn new(_x: i32, _y: i32) -> MyVec {
        MyVec{ x: _x, y: _y}
    }
}

impl Add for MyVec {
    type Output = Self;
    fn add(self, rhs: MyVec) -> MyVec {
        MyVec {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for MyVec {
    type Output = Self;
    fn sub(self, rhs: MyVec) -> MyVec {
        MyVec {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}



fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
