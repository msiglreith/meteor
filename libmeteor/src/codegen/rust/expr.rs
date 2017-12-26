
use expr::{Pat, __ExprBlock, __ExprTuple, __Ref, __RefMut};
use super::*;

impl<T> __ExprBlock<Expr<T>> for RustGen {
    fn __stmnt_local(&mut self, pat: Pat, expr: Expr<T>) -> Expr<T> {
        let mut pattern = Tokens::new();
        pat.to_tokens(&mut pattern);

        self.cur_scope().append(quote!{ let #pattern = });
        self.cur_scope().append(expr.0.clone());
        self.cur_scope().append(";");
        Expr::new(quote! { #pattern })
    }

    fn __expr(&mut self, expr: Expr<T>) -> Expr<T> {
        let mut scope = self.scopes.pop().unwrap();
        scope.append(expr.0);
        scope.append("}");
        Expr::new(scope)
    }
}

impl<A, B> __ExprTuple<(Expr<A>, Expr<B>)> for RustGen {
    type Output = Expr<(A, B)>;
    fn __expr(&mut self, tup: (Expr<A>, Expr<B>)) -> Expr<(A, B)> {
        let t0 = (tup.0).0;
        let t1 = (tup.1).0;

        Expr::new(quote! { (#t0, #t1) })
    }
}

impl<A, B, C> __ExprTuple<(Expr<A>, Expr<B>, Expr<C>)> for RustGen {
    type Output = Expr<(A, B, C)>;
    fn __expr(&mut self, tup: (Expr<A>, Expr<B>, Expr<C>)) -> Expr<(A, B, C)> {
        let t0 = (tup.0).0;
        let t1 = (tup.1).0;
        let t2 = (tup.2).0;

        Expr::new(quote! { (#t0, #t1, #t2) })
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
