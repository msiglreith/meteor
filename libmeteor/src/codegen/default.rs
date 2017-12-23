//! Virtualization implementation for normal, 'unstaged' Rust
use quote::Tokens;

use std::cmp::{PartialEq};
use std::ops::{Add, Sub, Not};

use expr::{__ExprBlock};
use ops::{__Not, __PartialEq, __Add, __Sub};
use super::Codegen;

impl<CG, T> __Not<T> for CG
where
    CG: Codegen,
    T: Not,
{
    type Output = <T as Not>::Output;
    fn not(&mut self, expr: T) -> Self::Output {
        Not::not(expr)
    }
}

impl<CG, LHS, RHS> __PartialEq<LHS, RHS> for CG
where
    CG: Codegen,
    LHS: PartialEq<RHS>,
{
    type Output = bool;
    fn eq(&mut self, lhs: &LHS, rhs: &RHS) -> bool {
        PartialEq::eq(lhs, rhs)
    }
    fn ne(&mut self, lhs: &LHS, rhs: &RHS) -> bool {
        PartialEq::ne(lhs, rhs)
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

impl<CG, T> __ExprBlock<T> for CG
where
    CG: Codegen,
{
    default fn __stmnt_local(&mut self, expr: T) -> T {
        expr
    }
}
