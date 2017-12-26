//! Virtualization implementation for normal, 'unstaged' Rust

use quote::Tokens;

use std::cmp::{PartialEq};
use std::mem;
use std::ops::{Add, Sub, Not};

use codegen::{__Codegen, Tokenize};
use expr::{Pat, __ExprBlock, __ExprTuple, __Ref, __RefMut};
use ops::{__Not, __PartialEq, __Add, __Sub};

impl<CG, T> __Not<T> for CG
where
    CG: __Codegen,
    T: Not,
{
    type Output = <T as Not>::Output;
    fn __not(&mut self, expr: T) -> Self::Output {
        Not::not(expr)
    }
}

impl<CG, LHS, RHS> __PartialEq<LHS, RHS> for CG
where
    CG: __Codegen,
    LHS: PartialEq<RHS>,
{
    type Output = bool;
    fn __eq(&mut self, lhs: &LHS, rhs: &RHS) -> bool {
        PartialEq::eq(lhs, rhs)
    }
    fn __ne(&mut self, lhs: &LHS, rhs: &RHS) -> bool {
        PartialEq::ne(lhs, rhs)
    }
}

impl<CG, LHS, RHS> __Add<LHS, RHS> for CG
where
    CG: __Codegen,
    LHS: Add<RHS>,
{
    type Output = <LHS as Add<RHS>>::Output;
    fn __add(&mut self, lhs: LHS, rhs: RHS) -> Self::Output {
        Add::add(lhs, rhs)
    }
}

impl<CG, LHS, RHS> __Sub<LHS, RHS> for CG
where
    CG: __Codegen,
    LHS: Sub<RHS>,
{
    type Output = <LHS as Sub<RHS>>::Output;
    fn __sub(&mut self, lhs: LHS, rhs: RHS) -> Self::Output {
        Sub::sub(lhs, rhs)
    }
}

impl<CG, T> __ExprBlock<T> for CG
where
    CG: __Codegen,
{
    default fn __expr(&mut self, expr: T) -> T {
        expr
    }
    default fn __stmnt_local(&mut self, _: Pat, expr: T) -> T {
        expr
    }
}

default impl<CG, T> __ExprTuple<T> for CG
where
    CG: __Codegen,
{
    default type Output = T;
    default fn __expr(&mut self, tup: T) -> Self::Output {
        // !DANGER!
        // This works ****only**** when we always specialize both,
        // Output and __expr.
        unsafe { mem::transmute_copy(&tup) }
    }
}

default impl<T> Tokenize for T {
    fn tokens(self) -> Tokens {
        Tokens::new()
    }
}

