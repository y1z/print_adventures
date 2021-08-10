use std::ops::*;

///
/// Trait's
///

/// To indicate that a type 'T' implements basic arithmetic functions.
/// : Add<Output = T
pub trait BasicArithmetic<T>:
  Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy
{
  type Output;
}

/// To indicate that a type 'T' implements basic arithmetic functions and the remainder function.
pub trait BasicArithmeticWithRem<T>: BasicArithmetic<T> + Rem<Output = T> {}

/// To indicate that the type implement the "+=" "-=" "*=" and "/=" functions.
pub trait AdvanceArithmetic<T>:
  BasicArithmetic<T> + AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T>
{
}

/// To indicate that the type implement the "+=" "-=" "*=" and "/=" functions plus remainder add function "%=".
pub trait AdvanceArithmeticWithRem<T>:
  BasicArithmeticWithRem<T> + AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T> + RemAssign<T>
{
}

/// To indicate that a type implements the remainder operations.
pub trait RemainderArithmetic<T>: Rem<T> + RemAssign<T> + Copy {}

/// To indicate that a type implement all the arithmetic functions
pub trait FullArithmetic<T>: AdvanceArithmetic<T> + RemainderArithmetic<T> {}

/// Used to indicate that your type can use the square root function
pub trait SquareRoot<T, U = T> {
  fn do_sqrt(&self) -> U;
}

/// Used to indicate that you type can do this
/// ```rust
/// let x = 5;
/// let x_squared = x * x
/// ```
pub trait Squared<T, U = T> {
  fn do_squared(&self) -> U;
}

///
/// Implementation of traits for primitive types
///

impl SquareRoot<f32, f32> for f32 {
  fn do_sqrt(&self) -> f32 {
    self.sqrt()
  }
}

impl SquareRoot<f64, f64> for f64 {
  fn do_sqrt(&self) -> f64 {
    self.sqrt()
  }
}

impl SquareRoot<i32, f32> for i32 {
  fn do_sqrt(&self) -> f32 {
    (*self as f32).sqrt()
  }
}

impl SquareRoot<i64, f64> for i64 {
  fn do_sqrt(&self) -> f64 {
    (*self as f64).sqrt()
  }
}

impl SquareRoot<u32, f32> for u32 {
  fn do_sqrt(&self) -> f32 {
    (*self as f32).sqrt()
  }
}
