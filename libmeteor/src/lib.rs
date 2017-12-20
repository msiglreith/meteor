#![feature(specialization)]

#[macro_use]
extern crate quote;

pub mod codegen;
pub mod expr;
pub mod ops;

use quote::Tokens;
