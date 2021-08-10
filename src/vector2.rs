use crate::util::util_traits::{BasicArithmetic, SquareRoot};
use std::any::{Any, TypeId};
use std::ops::*;

/// Represents a 2d mathematical vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct vector2<T> {
  x: T,
  y: T,
}

///
/// Implementations
///
impl<T> vector2<T>
where
  T: BasicArithmetic<T> + SquareRoot<T, T>,
{
  pub fn mag_sqr(&self) -> T {
    (self.x * self.x) + (self.y * self.y)
  }

  pub fn mag(&self) -> T {
    return self.mag().do_sqrt();
  }
}

impl<T: Add<Output = T>> Add for vector2<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    vector2::<T> {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl<T: Sub<Output = T>> Sub for vector2<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    vector2::<T> {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl<T: Mul<Output = T>> Mul for vector2<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self::Output {
    return vector2::<T> {
      x: self.x * other.x,
      y: self.y * other.y,
    };
  }
}

impl<T: Div<Output = T>> Div for vector2<T> {
  type Output = Self;
  fn div(self, other: Self) -> Self::Output {
    vector2::<T> {
      x: self.x / other.x,
      y: self.y / other.y,
    }
  }
}

impl<T: Neg<Output = T>> Neg for vector2<T> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    vector2::<T> {
      x: -self.x,
      y: -self.y,
    }
  }
}

/**
 * Pre-made types.
 */
pub type vector2u = vector2<u32>;
pub type vector2i = vector2<i32>;
pub type vector2f = vector2<f32>;
pub type vector2d = vector2<f64>;
