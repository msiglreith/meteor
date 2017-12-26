//! Virtualization implementation for 'staged' Rust

use quote::Tokens;
use quote::ToTokens;

use std::marker::PhantomData;

use codegen::{__Codegen, Tokenize};

mod expr;
mod ops;

pub struct RustGen {
    scopes: Vec<Tokens>,
}

impl RustGen {
    pub fn new() -> Self {
        RustGen {
            scopes: Vec::new()
        }
    }

    fn cur_scope(&mut self) -> &mut Tokens {
        self.scopes.last_mut().unwrap()
    }
}

impl __Codegen for RustGen {
    fn begin_scope(&mut self) {
        self.scopes.push(Tokens::new());
        self.cur_scope().append("{");
    }
    fn end_scope(&mut self) {
        // Scope ending handled prior on the last expr
    }
}

pub struct Repr<T>(Tokens, PhantomData<T>);
pub struct Expr<T>(Tokens, PhantomData<T>);

impl<T> Repr<T> {
    pub unsafe fn new(tt: Tokens) -> Self {
        Repr(tt, PhantomData)
    }
}

impl Expr<()> {
    pub fn empty() -> Expr<()> {
        Expr(Tokens::new(), PhantomData)
    }
}

impl<T> Expr<T> {
    fn new(tt: Tokens) -> Self {
        Expr(tt, PhantomData)
    }
}

impl<T: ToTokens> Expr<T> {
    pub fn lit(v: T) -> Self {
        Expr::new(quote! { #v })
    }
}

impl<T> Tokenize for Expr<T> {
    fn tokens(self) -> Tokens {
        self.0
    }
}
