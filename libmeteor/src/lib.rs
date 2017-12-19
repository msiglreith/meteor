#![feature(specialization)]

#[macro_use]
extern crate quote;

pub mod codegen;
pub mod ops;

use quote::Tokens;

pub trait __ExprBlock {
    type Return;
    fn __stmnt_local(self, tokens: &mut Tokens) -> Self::Return;
}

impl<F, T> __ExprBlock for F where F: FnOnce() -> T {
    type Return = T;
    default fn __stmnt_local(self, tokens: &mut Tokens) -> Self::Return {
        self()
    }
}
