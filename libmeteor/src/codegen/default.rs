//! Virtualization implementation for normal, 'unstaged' Rust
use quote::Tokens;

use std::cmp::{PartialEq};
use std::ops::{Add, Sub, Not};

use expr::{__ExprBlock};
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

impl<F, T> __ExprBlock for F where F: FnOnce() -> T {
    type Return = T;
    default fn __stmnt_local(self, tokens: &mut Tokens) -> Self::Return {
        self()
    }
}
