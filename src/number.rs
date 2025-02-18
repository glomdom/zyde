use std::{
    fmt::Display,
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
    + From<i32>
{
}

impl Number for i32 {}