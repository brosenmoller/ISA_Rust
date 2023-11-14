use std::ops;

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}


impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub const fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
    
    pub fn normalize(&mut self) -> &Vector2
    {
        if self.x == 0.0 && self.y == 0.0 { return self; }

        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        
        return self;
    }
    
    pub fn normalized(&mut self) -> Vector2
    {
        if self.x == 0.0 && self.y == 0.0 { return Vector2 { x: self.x, y: self.y }; }

        let magnitude = self.magnitude();

        return Vector2 { x: self.x / magnitude, y: self.y / magnitude };
    }
    
    pub fn magnitude(&self) -> f32
    {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
    
    pub fn sqr_magnitude(&self) -> f32
    {
        return self.x.powi(2) + self.y.powi(2);
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 
    {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 
    {
        Vector2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl ops::Mul<f32> for Vector2 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self { 
            x: self.x * other, 
            y: self.y * other 
        }
    }
}