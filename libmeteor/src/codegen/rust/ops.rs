
use std::ops::{Add, Not};

use ops::*;
use super::*;

impl<'a, T, U> __Add<&'a Expr<T>, &'a Expr<U>> for RustGen
where
    T: Add<U>,
{
    type Output = Expr<<T as Add<U>>::Output>;
    fn __add(&mut self, lhs: &'a Expr<T>, rhs: &'a Expr<U>) -> Self::Output {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        Expr::new(quote! { #lhs + #rhs })
    }
}

impl<'a, T, U> __Add<Expr<T>, Expr<U>> for RustGen
where
    T: Add<U>,
{
    type Output = Expr<<T as Add<U>>::Output>;
    fn __add(&mut self, lhs: Expr<T>, rhs: Expr<U>) -> Self::Output {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        Expr::new(quote! { #lhs + #rhs })
    }
}

impl<T> __Not<Repr<T>> for RustGen
where
    T: Not,
{
    type Output = Expr<T::Output>;
    fn __not(&mut self, expr: Repr<T>) -> Self::Output {
        let r = expr.0;
        Expr::new(quote! { !#r })
    }
}

impl<T> __Not<Expr<T>> for RustGen
where
    T: Not,
{
    type Output = Expr<T::Output>;
    fn __not(&mut self, expr: Expr<T>) -> Self::Output {
        let r = expr.0;
        Expr::new(quote! { !#r })
    }
}

impl<T, U> __PartialEq<Expr<T>, Expr<U>> for RustGen
where
    T: PartialEq<U>,
{
    type Output = Expr<bool>;
    fn __eq(&mut self, lhs: &Expr<T>, rhs: &Expr<U>) -> Expr<bool> {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        Expr::new(quote! { #lhs == #rhs })
    }
    fn __ne(&mut self, lhs: &Expr<T>, rhs: &Expr<U>) -> Expr<bool> {
        let lhs = &lhs.0;
        let rhs = &rhs.0;
        Expr::new(quote! { #lhs != #rhs })
    }
}

impl<T> __Assign<Repr<T>, Repr<T>> for RustGen {
    type Output = Expr<()>;
    fn __assign(&mut self, lhs: Repr<T>, rhs: Repr<T>) -> Expr<()> {
        let lhs = lhs.0;
        let rhs = rhs.0;
        Expr::new(quote! { #lhs = #rhs })
    }
}

impl<T> __Assign<Repr<T>, Expr<T>> for RustGen {
    type Output = Expr<()>;
    fn __assign(&mut self, lhs: Repr<T>, rhs: Expr<T>) -> Expr<()> {
        let lhs = lhs.0;
        let rhs = rhs.0;
        Expr::new(quote! { #lhs = #rhs })
    }
}
