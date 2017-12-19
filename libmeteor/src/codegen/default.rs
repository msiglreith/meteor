//! Virtualization implementation for normal, 'unstaged' Rust

use std::cmp::{PartialEq};
use std::ops::{Add, Sub, Not};

use ops::{__Not, __PartialEq, __Add, __Sub};

impl<T> __Not for T
where
    T: Not,
{
    type Output = <T as Not>::Output;
    fn not(self) -> Self::Output {
        Not::not(self)
    }
}

impl<T, RHS> __PartialEq<RHS> for T
where
    T: PartialEq<RHS>,
{
    type Output = bool;
    fn eq(&self, rhs: &RHS) -> bool {
        PartialEq::eq(self, rhs)
    }
}

impl<T, RHS> __Add<RHS> for T
where
    T: Add<RHS>,
{
    type Output = <T as Add<RHS>>::Output;
    fn add(self, rhs: RHS) -> Self::Output {
        Add::add(self, rhs)
    }
}

impl<T, RHS> __Sub<RHS> for T
where
    T: Sub<RHS>,
{
    type Output = <T as Sub<RHS>>::Output;
    fn sub(self, rhs: RHS) -> Self::Output {
        Sub::sub(self, rhs)
    }
}
