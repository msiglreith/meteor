
use quote::Tokens;
use quote::ToTokens;

pub enum Pat {
    Wild,
    Ident(&'static str),
}

impl ToTokens for Pat {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Pat::Wild => {
                tokens.append("_");
            }
            Pat::Ident(ref ident) => {
                tokens.append(ident);
            }
        }
    }
}

pub trait __ExprBlock<T> {
    fn __expr(&mut self, expr: T) -> T;
    fn __stmnt_local(&mut self, pat: Pat, expr: T) -> T;
}

pub trait __ExprTuple<T> {
    type Output;
    fn __expr(&mut self, tup: T) -> Self::Output;
}

pub trait __Ref<'a, T> {
    type Output;
    fn __ref(&'a self) -> Self::Output;
}

pub trait __RefMut<'a, T> {
    type Output;
    fn __mut(&'a mut self) -> Self::Output;
}
