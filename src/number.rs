use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

pub trait Number:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + Display
    + Debug
    + From<i32>
    + std::fmt::Display
{
}

impl Number for i32 {}
impl Number for f64 {}
