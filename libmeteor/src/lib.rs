#![feature(specialization)]

#[macro_use]
extern crate quote;

use quote::Tokens;
use std::marker::PhantomData;
use std::ops::Not;

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

pub trait __Not {
    type Output;
    fn not(self) -> Self::Output;
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

pub trait __PartialEq<RHS, Ret>
where
    Ret: __Not<Output=Ret>,
{
    fn eq(&self, RHS) -> Ret;
    fn ne(&self, other: RHS) -> Ret { self.eq(other).not() }
}

impl<T, U> __PartialEq<Repr<U>, Expr<bool>> for Repr<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, repr: Repr<U>) -> Expr<bool> {
        let lhs = &self.0;
        let rhs = &repr.0;
        Expr::new(quote! { #lhs == #rhs })
    }
}

impl __PartialEq<Expr<bool>, Expr<bool>> for Repr<bool> {
    fn eq(&self, expr: Expr<bool>) -> Expr<bool> {
        let lhs = &self.0;
        let rhs = &expr.0;
        Expr::new(quote! { #lhs == #rhs })
    }
}

pub trait __Assign<RHS> {
    fn assign(self, RHS) -> Stmt;
}

impl<T> __Assign<Repr<T>> for Repr<T> {
    fn assign(self, rhs: Repr<T>) -> Stmt {
        let lhs = self.0;
        let rhs = rhs.0;
        Stmt::new(quote! { #lhs = #rhs })
    }
}

impl<T> __Assign<Expr<T>> for Repr<T> {
    fn assign(self, rhs: Expr<T>) -> Stmt {
        let lhs = self.0;
        let rhs = rhs.0;
        Stmt::new(quote! { #lhs = #rhs })
    }
}

pub trait __Ref<T> {
    fn __ref<'a>(&'a self) -> Expr<&'a T>;
}

impl<T> __Ref<T> for Repr<T> {
    fn __ref<'a>(&'a self) -> Expr<&'a T> {
        let r = &self.0;
        Expr::new(quote! { &#r })
    }
}

pub trait __RefMut<T> {
    fn __mut<'a>(&'a mut self) -> Expr<&'a mut T>;
}

impl<T> __RefMut<T> for Repr<T> {
    fn __mut<'a>(&'a mut self) -> Expr<&'a mut T> {
        let r = &self.0;
        Expr::new(quote! { &mut #r })
    }
}

#[test]
fn basic() {
    let x = unsafe { Repr::<u32>::new(quote!{a}) };
    let y = unsafe { Repr::<u32>::new(quote!{b}) };
    let z = unsafe { Repr::<bool>::new(quote!{c}) };

    assert_eq!(z.assign(x.eq(y)).0.to_string(), "c = a == b");
}

pub trait __ExprBlock {
    type Return;
    fn __stmnt_local(self, tokens: &mut Tokens) -> Self::Return;
    // fn __stmnt_expr(self, tokens: &mut Tokens);
}

impl<F, T> __ExprBlock for F where F: FnOnce() -> T {
    type Return = T;
    default fn __stmnt_local(self, tokens: &mut Tokens) -> Self::Return {
        self()
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
fn expr_block() {
    let mut __tokens = Tokens::new();

    let x = unsafe { Repr::<u32>::new(quote!{a}) };
    let y = unsafe { Repr::<u32>::new(quote!{b}) };

    {
        __tokens.append("{");
        let k =__ExprBlock::__stmnt_local(|| { 2 + 4 }, &mut __tokens);
        let x =__ExprBlock::__stmnt_local(|| { x.eq(y) }, &mut __tokens);
        __tokens.append("}");
    }

    assert_eq!(__tokens.to_string(), "{ let x = a == b ; }");
}
