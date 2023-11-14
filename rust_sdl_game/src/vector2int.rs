use std::{ops, cmp};

#[derive(Clone)]
pub struct Vector2Int 
{
    pub x: i32,
    pub y: i32,
}

impl Vector2Int {
    pub const fn new(x: i32, y: i32) -> Vector2Int {
        return Vector2Int { x, y }
    }
    
    pub const fn zero() -> Vector2Int {
        return Vector2Int { x: 0, y: 0 }
    }
    
    pub const fn default() -> Vector2Int {
        return Vector2Int { x: 0, y: 0 }
    }
}

impl ops::Add<Vector2Int> for Vector2Int {
    type Output = Vector2Int;

    fn add(self, other: Vector2Int) -> Vector2Int 
    {
        return Vector2Int { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Sub<Vector2Int> for Vector2Int {
    type Output = Vector2Int;

    fn sub(self, other: Vector2Int) -> Vector2Int 
    {
        return Vector2Int { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::AddAssign for Vector2Int {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::SubAssign for Vector2Int {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl ops::Mul<i32> for Vector2Int {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self { 
            x: self.x * other, 
            y: self.y * other 
        }
    }
}

impl cmp::PartialEq for Vector2Int {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}