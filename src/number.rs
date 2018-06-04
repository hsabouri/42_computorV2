use std::ops::{Add, Sub, Mul, Div, Rem};
use std::f32;

#[derive(Clone, Copy)]
pub struct Number {
    content: f32,
}

impl Number {
    pub fn new(n: f32) -> Number {
        Number {
            content: n
        }
    }

    pub fn verify(self) -> Result<Number, String> {
        if self.content <= 0.0 + f32::EPSILON && self.content >= 0.0 - f32::EPSILON {
            Err(format!("is 0"))
        } else {
            Ok(self)
        }
    }

    pub fn sqrt(&self) -> Result<Number, String> {
        match self.content {
            a if a <= 0.0 + f32::EPSILON => Err(format!("sqrt on an invalid number: {}", a)),
            a => Ok(Number {content: a.sqrt()}),
        }
    }
}

impl Add for Number {
    type Output = Result<Number, String>;

    fn add(self, other: Number) -> Result<Number, String> {
        let left = self.content;
        let right = other.content;

        Ok(Number {content: left + right})
    }
}

impl Sub for Number {
    type Output = Result<Number, String>;

    fn sub(self, other: Number) -> Result<Number, String> {
        let left = self.content;
        let right = other.content;

        Ok(Number {content: left - right})
    }
}

impl Mul for Number {
    type Output = Result<Number, String>;

    fn mul(self, other: Number) -> Result<Number, String> {
        let left = self.content;
        let right = other.content;

        Ok(Number {content: left * right})
    }
}

impl Div for Number {
    type Output = Result<Number, String>;

    fn div(self, other: Number) -> Result<Number, String> {
        let left = self.content;
        let right = other.verify()?.content;

        Ok(Number {content: left / right})
    }
}
