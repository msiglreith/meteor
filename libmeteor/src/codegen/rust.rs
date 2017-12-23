//! Virtualization implementation for 'staged' Rust

use quote::Tokens;
use std::marker::PhantomData;
use std::ops::Not;

use expr::__ExprBlock;
use ops::*;

use super::Codegen;

pub struct RustGen {
    tokens: Tokens,
}

impl Codegen for RustGen { }

pub struct Lit<T>(Tokens, PhantomData<T>);

impl Lit<u8> {
    pub fn new(v: u8) -> Self {
        Lit(quote! { #v }, PhantomData)
    }
}

pub struct Repr<T>(Tokens, PhantomData<T>);
pub struct Expr<T>(Tokens, PhantomData<T>);
pub struct Stmt(Tokens);

impl<T> Repr<T> {
    pub unsafe fn new(tt: Tokens) -> Self {
        Repr(tt, PhantomData)
    }
}

impl<T> Expr<T> {
    fn new(tt: Tokens) -> Self {
        Expr(tt, PhantomData)
    }
}

impl Stmt {
    fn new(tt: Tokens) -> Self {
        Stmt(tt)
    }
}

impl<T> __Not<Repr<T>> for RustGen
where
    T: Not,
{
    type Output = Expr<T::Output>;
    fn not(&mut self, expr: Repr<T>) -> Self::Output {
        let r = expr.0;
        Expr::new(quote! { !#r })
    }
}

impl<T> __Not<Expr<T>> for RustGen
where
    T: Not,
{
    type Output = Expr<T::Output>;
    fn not(&mut self, expr: Expr<T>) -> Self::Output {
        let r = expr.0;
        Expr::new(quote! { !#r })
    }
}

impl<T, U> __PartialEq<Expr<T>, Expr<U>> for RustGen
where
    T: PartialEq<U>,
{
    type Output = Expr<bool>;
    fn eq(&mut self, lhs: &Expr<T>, rhs: &Expr<U>) -> Expr<bool> {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        Expr::new(quote! { #lhs == #rhs })
    }
    fn ne(&mut self, lhs: &Expr<T>, rhs: &Expr<U>) -> Expr<bool> {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        Expr::new(quote! { #lhs != #rhs })
    }
}

impl<T> __Assign<Repr<T>> for Repr<T> {
    type Output = Stmt;
    fn assign(self, rhs: Repr<T>) -> Stmt {
        let lhs = self.0;
        let rhs = rhs.0;
        Stmt::new(quote! { #lhs = #rhs })
    }
}

impl<T> __Assign<Expr<T>> for Repr<T> {
    type Output = Stmt;
    fn assign(self, rhs: Expr<T>) -> Stmt {
        let lhs = self.0;
        let rhs = rhs.0;
        Stmt::new(quote! { #lhs = #rhs })
    }
}

impl<'a, T: 'a> __Ref<'a, T> for Repr<T> {
    type Output = Expr<&'a T>;
    fn __ref(&'a self) -> Expr<&'a T> {
        let r = &self.0;
        Expr::new(quote! { &#r })
    }
}

impl<'a, T: 'a> __RefMut<'a, T> for Repr<T> {
    type Output = Expr<&'a mut T>;
    fn __mut(&'a mut self) -> Expr<&'a mut T> {
        let r = &self.0;
        Expr::new(quote! { &mut #r })
    }
}

impl<T> __ExprBlock<Expr<T>> for RustGen {
    fn __stmnt_local(&mut self, expr: Expr<T>) -> Expr<T> {
        self.tokens.append(quote!{ let x = }); // TODO
        self.tokens.append(expr.0.clone());
        self.tokens.append(";");
        expr
    }
}

#[test]
fn basic() {
    let x = unsafe { Repr::<u32>::new(quote!{a}) };
    let y = unsafe { Repr::<u32>::new(quote!{b}) };
    let z = unsafe { Repr::<bool>::new(quote!{c}) };

    assert_eq!(z.assign(x.eq(&y)).0.to_string(), "c = a == b");
}

#[test]
fn expr_block() {
    let mut __tokens = Tokens::new();

    let x = unsafe { Repr::<u32>::new(quote!{a}) };
    let y = unsafe { Repr::<u32>::new(quote!{b}) };

    {
        __tokens.append("{");
        let k =__ExprBlock::__stmnt_local(2 + 4, &mut __tokens);
        let y =__ExprBlock::__stmnt_local(x.eq(&y), &mut __tokens);
        __tokens.append("}");
    }

    assert_eq!(__tokens.to_string(), "{ let x = a == b ; }");
}
