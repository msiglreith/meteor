//! Virtualization implementation for normal, 'unstaged' Rust

use std::cmp::{PartialEq};
use std::ops::{Not};

use ops::{__Not, __PartialEq};

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
