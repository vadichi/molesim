use crate::math::Real;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    pub x: Real,
    pub y: Real,
}

impl Vector2 {
    pub fn new(x: Real, y: Real) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }
}

impl Vector2 {
    pub fn magnitude(self) -> Real {
        (self * self).sqrt()
    }

    pub fn distance(self, other: Vector2) -> Real {
        (self - other).magnitude()
    }

    pub fn normalized(self) -> Vector2 {
        self / self.magnitude()
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Self::Output {
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

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Self::Output {
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

    fn mul(self, scalar: Real) -> Self::Output {
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

    fn div(self, scalar: Real) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl std::ops::DivAssign<Real> for Vector2 {
    fn div_assign(&mut self, scalar: Real) {
        *self *= 1.0 / scalar;
    }
}

impl std::ops::Mul<Vector2> for Vector2 {
    type Output = Real;

    fn mul(self, rhs: Vector2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
