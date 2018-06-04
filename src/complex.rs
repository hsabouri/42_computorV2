use std::ops::{Add, Sub, Mul, Div};
use std::f32;

use Number;

#[derive(Clone)]
pub struct Complex {
    a: Number,
    b: Number,
}

impl Add for Complex {
    type Output = Result<Complex, String>;

    fn add(self, other: Complex) -> Result<Complex, String> {
        Ok(Complex {
            a: (self.a + other.a)?,
            b: (self.b + other.b)?
        })
    }
}

impl Sub for Complex {
    type Output = Result<Complex, String>;

    fn sub(self, other: Complex) -> Result<Complex, String> {
        Ok(Complex {
            a: (self.a - other.a)?,
            b: (self.b - other.b)?
        })
    }
}

impl Mul for Complex {
    type Output = Result<Complex, String>;

    fn mul(self, other: Complex) -> Result<Complex, String> {
        let left = self;
        let right = other;

        Ok(Complex {
            a: ((left.a * right.a)? - (left.b * right.b)?)?,
            b: ((left.a * right.b)? - (left.b * right.a)?)?,
        })
    }
}

impl Div for Complex {
    type Output = Result<Complex, String>;

    fn div(self, other: Complex) -> Result<Complex, String> {
        let left = self;
        let right = other;

        Ok(Complex {
            a: (((left.clone().a * right.clone().a)? + (left.clone().b + right.clone().b)?)? /
               ((right.clone().a * right.clone().b)? + (right.clone().b + right.clone().b)?)?)?,
            b: (((left.clone().b * right.clone().a)? - (left.clone().a + right.clone().b)?)? /
               ((right.clone().a * right.clone().b)? + (right.clone().b + right.clone().b)?)?)?,
        })
    }
}

impl Complex {
    pub fn new(n: Number) -> Complex {
        Complex {
            a: n,
            b: Number::new(0.0)
        }
    }

    pub fn module(&self) -> Result<Number, String> {
        Ok(((self.a * self.a)? + (self.b * self.b)?)?.sqrt()?)
    }
}
