//! Virtualization implementation for 'staged' Rust

use quote::Tokens;
use std::marker::PhantomData;
use std::ops::Not;

use expr::__ExprBlock;
use ops::*;

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

impl<T> __Not for Repr<T>
where
    T: Not,
{
    type Output = Expr<T::Output>;
    fn not(self) -> Self::Output {
        let r = self.0;
        Expr::new(quote! { !#r })
    }
}

impl<T> __Not for Expr<T>
where
    T: Not,
{
    type Output = Expr<T::Output>;
    fn not(self) -> Self::Output {
        let e = self.0;
        Expr::new(quote! { !#e })
    }
}

impl<T, U> __PartialEq<Repr<U>> for Repr<T>
where
    T: PartialEq<U>,
{
    type Output = Expr<bool>;
    fn eq(&self, repr: &Repr<U>) -> Expr<bool> {
        let lhs = &self.0;
        let rhs = &repr.0;
        Expr::new(quote! { #lhs == #rhs })
    }
}

impl __PartialEq<Expr<bool>> for Repr<bool> {
    type Output = Expr<bool>;
    fn eq(&self, expr: &Expr<bool>) -> Expr<bool> {
        let lhs = &self.0;
        let rhs = &expr.0;
        Expr::new(quote! { #lhs == #rhs })
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

impl<F, T> __ExprBlock for F where F: FnOnce() -> Expr<T> {
    fn __stmnt_local(self, tokens: &mut Tokens) -> Self::Return {
        let ret = self();
        tokens.append(quote!{ let x = });
        tokens.append(ret.0.clone());
        tokens.append(";");
        ret
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
        let k =__ExprBlock::__stmnt_local(|| { 2 + 4 }, &mut __tokens);
        let x =__ExprBlock::__stmnt_local(|| { x.eq(&y) }, &mut __tokens);
        __tokens.append("}");
    }

    assert_eq!(__tokens.to_string(), "{ let x = a == b ; }");
}
