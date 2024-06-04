use crate::math::Real;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MVector2 {
    x: Real,
    y: Real,
}

impl MVector2 {
    pub fn new(x: Real, y: Real) -> MVector2 {
        MVector2 { x, y }
    }

    pub fn zero() -> MVector2 {
        MVector2 { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> Real {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn x(&self) -> Real {
        self.x
    }

    pub fn y(&self) -> Real {
        self.y
    }
}

impl std::ops::Add for MVector2 {
    type Output = MVector2;

    fn add(self, other: MVector2) -> MVector2 {
        MVector2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::AddAssign for MVector2 {
    fn add_assign(&mut self, other: MVector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Neg for MVector2 {
    type Output = MVector2;

    fn neg(self) -> MVector2 {
        MVector2::new(-self.x, -self.y)
    }
}

impl std::ops::Sub for MVector2 {
    type Output = MVector2;

    fn sub(self, other: MVector2) -> MVector2 {
        self + (-other)
    }
}

impl std::ops::SubAssign for MVector2 {
    fn sub_assign(&mut self, other: MVector2) {
        *self += -other;
    }
}

impl std::ops::Mul<Real> for MVector2 {
    type Output = MVector2;

    fn mul(self, scalar: Real) -> MVector2 {
        MVector2::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Mul<MVector2> for Real {
    type Output = MVector2;

    fn mul(self, vector: MVector2) -> MVector2 {
        vector * self
    }
}

impl std::ops::MulAssign<Real> for MVector2 {
    fn mul_assign(&mut self, scalar: Real) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::Div<Real> for MVector2 {
    type Output = MVector2;

    fn div(self, scalar: Real) -> MVector2 {
        self * (1.0 / scalar)
    }
}

impl std::ops::DivAssign<Real> for MVector2 {
    fn div_assign(&mut self, scalar: Real) {
        *self *= 1.0 / scalar;
    }
}
