use std::ops::{Add,Sub};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct MyVec {
    pub x: i32,
    pub y: i32
}

impl MyVec {
    pub fn new(_x: i32, _y: i32) -> MyVec {
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