use crate::math::Real;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    x: Real,
    y: Real,
}

impl Vector2 {
    pub fn new(x: Real, y: Real) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    pub fn x(&self) -> Real {
        self.x
    }

    pub fn y(&self) -> Real {
        self.y
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        self + (-other)
    }
}

impl std::ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Vector2) {
        *self += -other;
    }
}

impl std::ops::Mul<Real> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: Real) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Mul<Vector2> for Real {
    type Output = Vector2;

    fn mul(self, vector: Vector2) -> Vector2 {
        vector * self
    }
}

impl std::ops::MulAssign<Real> for Vector2 {
    fn mul_assign(&mut self, scalar: Real) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::Div<Real> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: Real) -> Vector2 {
        self * (1.0 / scalar)
    }
}

impl std::ops::DivAssign<Real> for Vector2 {
    fn div_assign(&mut self, scalar: Real) {
        *self *= 1.0 / scalar;
    }
}
